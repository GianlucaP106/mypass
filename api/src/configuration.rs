use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use crate::{error::Error, util};

pub async fn connect() -> Result<DatabaseConnection, Error> {
    let path_to_db = get_db_path()?;
    let db_url = format!("sqlite://{}", path_to_db);
    let is_new = util::create_file(path_to_db.to_owned())?;
    let conn = Database::connect(db_url)
        .await
        .map_err(|_| "Failed to connect to data store")?;
    if is_new {
        Migrator::up(&conn, None)
            .await
            .map_err(|_| "Failed to push migration to data store")?;
    }
    Ok(conn)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    db: String,
}

pub fn get_db_path() -> Result<String, Error> {
    init_config()?;
    get_config().map(|c| c.db)
}

pub fn set_db_path(path: String) -> Result<(), Error> {
    init_config()?;
    set_config(path)
}

pub fn move_db(new_path: String) -> Result<(), Error> {
    let cur_path = get_db_path()?;
    fs::rename(cur_path, new_path).map_err(|_| "Failed to to move db file".to_owned())
}

fn get_config() -> Result<Configuration, Error> {
    let config_path = get_config_dir_path().map(|mut path| {
        path.push("config.json");
        path
    })?;

    let mut file: File = OpenOptions::new()
        .read(true)
        .open(config_path)
        .map_err(|_| "Failed to open config file")?;

    let mut config_str = String::new();
    file.read_to_string(&mut config_str)
        .map_err(|_| "Failed to read from configuration file")?;
    let config: Configuration = serde_json::from_str(&config_str)
            .map_err(|_| "Failed to read from configuration file (invalid configuration). The file has likely been tampered with. Fix the format or delete it to solve the issue")?;
    Ok(config)
}

fn set_config(db_path: String) -> Result<(), Error> {
    let config_path = get_config_dir_path().map(|mut path| {
        path.push("config.json");
        path
    })?;

    let mut file = File::create(config_path).map_err(|_| "Failed to create file")?;

    let new_config = Configuration { db: db_path };
    let config_str =
        serde_json::to_string_pretty(&new_config).map_err(|_| "Failed to serialize")?;
    file.write_all(config_str.as_bytes())
        .map_err(|_| "Failed to write to configuration file")?;

    Ok(())
}

fn get_config_dir_path() -> Result<PathBuf, Error> {
    util::get_home_env_var().map(|dir| {
        let mut path = PathBuf::new();
        path.push(dir);
        path.push(".mypass");
        path
    })
}

fn init_config() -> Result<(String, bool), Error> {
    let config_path = get_config_dir_path().map(|mut path| {
        path.push("config.json");
        path
    })?;
    let config_path = config_path.to_string_lossy().into_owned();
    let is_new = util::create_file(config_path.clone())?;
    if is_new {
        let default_db_path = get_config_dir_path().map(|mut p| {
            p.push("db.sqlite");
            p.to_string_lossy().into_owned()
        })?;
        set_config(default_db_path.clone())?;
    }
    Ok((config_path, is_new))
}
