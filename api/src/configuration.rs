use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{error::Error, util};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    db: String,
}

pub async fn connect() -> Result<DatabaseConnection, Error> {
    init_config()?;
    let path_to_db = get_db_path()?;
    connect_db(path_to_db).await
}

pub fn move_db(new_path: String) -> Result<(), Error> {
    let cur_path = get_db_path()?;
    let cur_path = Path::new(&cur_path);
    let full_new_path = Path::new(&new_path)
        .canonicalize()
        .map_err(|_| format!("Directory {new_path} is invalid"))?;

    let full_new_path = full_new_path.as_path();
    let full_new_path_str = full_new_path.to_string_lossy().into_owned();
    full_new_path
        .is_dir()
        .then_some(())
        .ok_or_else(|| format!("Directory {full_new_path_str} is invalid"))?;

    let db_path = full_new_path.join("db.sqlite");
    fs::rename(cur_path, db_path.as_path()).map_err(|_| "Failed to to move db file".to_owned())?;
    set_config(full_new_path.to_path_buf())
}

async fn connect_db(path: PathBuf) -> Result<DatabaseConnection, Error> {
    let path_to_db = path.to_string_lossy().into_owned();
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

pub fn get_db_path() -> Result<PathBuf, Error> {
    get_config().map(|c| PathBuf::from(c.db))
}

pub fn set_db_path(path: String) -> Result<(), Error> {
    let path = Path::new(&path)
        .canonicalize()
        .map_err(|_| format!("Directory {path} is invalid"))?;

    set_config(path)
}

fn init_config() -> Result<(), Error> {
    let config_path = get_config_dir_path().map(|mut path| {
        path.push("config.json");
        path
    })?;
    let config_path = config_path.to_string_lossy().into_owned();
    let is_new = util::create_file(config_path.clone())?;
    if is_new {
        let config_dir = get_config_dir_path()?;
        set_config(config_dir)?;
    }
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

fn set_config(mut db_path: PathBuf) -> Result<(), Error> {
    let config_path = get_config_dir_path().map(|mut path| {
        path.push("config.json");
        path
    })?;

    let mut file = File::create(config_path).map_err(|_| "Failed to create file")?;

    let default_db_path = {
        db_path.push("db.sqlite");
        db_path
    };

    let config = Configuration {
        db: default_db_path.to_string_lossy().into_owned(),
    };

    let config_str = serde_json::to_string_pretty(&config).map_err(|_| "Failed to serialize")?;
    file.write_all(config_str.as_bytes())
        .map_err(|_| "Failed to write to configuration file")?;

    Ok(())
}
