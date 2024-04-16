use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::util;

pub async fn connect() -> Result<DatabaseConnection, String> {
    let path_to_db = get_path_to_db()?;
    let db_url = format!("sqlite://{}", path_to_db);
    let is_new = util::create_file(path_to_db.to_owned())?;
    let conn = Database::connect(db_url)
        .await
        .map_err(|_| "Failed to connect to database")?;
    if is_new {
        Migrator::up(&conn, None)
            .await
            .map_err(|_| "Failed to push migration to database")?;
    }
    Ok(conn)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    db: String,
}

pub fn get_path_to_db() -> Result<String, String> {
    let home_dir: String = env::var("HOME").map_err(|_| "HOME env var not set.")?;
    let path_to_config = format!("{}/{}", home_dir, String::from(".mypass/config.json"));
    let is_new = util::create_file(path_to_config.to_owned())?;
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path_to_config)
        .map_err(|_| "Failed to open config file")?;
    if is_new {
        let default_db_path = format!("{}/{}", home_dir, String::from(".mypass/db.sqlite"));
        let new_config = Configuration {
            db: default_db_path.to_owned(),
        };
        let config_str =
            serde_json::to_string_pretty(&new_config).map_err(|_| "Failed to serialize")?;
        file.write_all(config_str.as_bytes())
            .map_err(|_| "Failed to write to configuration file")?;
        Ok(default_db_path)
    } else {
        let mut config_str = String::new();
        file.read_to_string(&mut config_str)
            .map_err(|_| "Failed to read from configuration file")?;
        let config: Configuration = serde_json::from_str(&config_str)
            .map_err(|_| "Failed to read from configuration file (invalid configuration). The file has likely been tampered with. Fix the format or delete it to solve the issue")?;
        Ok(config.db)
    }
}
