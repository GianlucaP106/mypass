use std::env;

use model::entities;

use crate::{
    crypto,
    entries::{self, create_entry},
    master, util,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryRecord {
    pub name: Option<String>,
    pub description: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub password: String,
}

impl EntryRecord {
    pub fn from_entry(
        entry: entities::entry::Model,
        master: entities::master::Model,
        master_password: String,
    ) -> Result<EntryRecord, String> {
        let decrypted_password =
            crypto::decrypt_password(master_password, entry.password, entry.id, master.id)?;
        Ok(EntryRecord {
            name: Some(entry.name),
            description: entry.description,
            username: entry.username,
            url: entry.url,
            password: decrypted_password,
        })
    }
}

pub async fn export_entries(master_password: String, path: Option<String>) -> Result<(), String> {
    let master = master::get_master().await.ok_or("Failed to get master")?;
    let path_to_csv: Result<String, String> = path.map_or_else(
        || {
            let home_dir: String = env::var("HOME").map_err(|_| "HOME env var not set.")?;
            let p = format!("{}/{}", home_dir, String::from(".mypass/entries.csv"));
            Ok(p)
        },
        Ok,
    );
    let path_to_csv: String = path_to_csv?;
    util::create_file(path_to_csv.to_owned())?;
    let entries: Vec<entities::entry::Model> = entries::get_all_entries().await?;
    let mut wtr =
        csv::Writer::from_path(path_to_csv).map_err(|_| "Error writing to csv".to_owned())?;

    let mut out: Vec<EntryRecord> = Vec::new();
    for ele in entries {
        out.push(EntryRecord::from_entry(
            ele.to_owned(),
            master.to_owned(),
            master_password.to_owned(),
        )?);
    }

    out.iter().for_each(|entry| {
        wtr.serialize(entry).ok();
    });

    wtr.flush().map_err(|_| "Error writing to csv")?;
    Ok(())
}

pub async fn import_entries(master_password: String, path: Option<String>) -> Result<(), String> {
    let path_to_csv: Result<String, String> = path.map_or_else(
        || {
            let home_dir: String = env::var("HOME").map_err(|_| "HOME env var not set.")?;
            let p = format!("{}/{}", home_dir, String::from(".mypass/entries.csv"));
            Ok(p)
        },
        Ok,
    );
    let path_to_csv: String = path_to_csv?;
    let mut rdr =
        csv::Reader::from_path(path_to_csv).map_err(|_| "Failed to read from provided path")?;
    for result in rdr.deserialize() {
        let record: EntryRecord = result.map_err(|e| format!("Failed to read entry {}", e))?;
        create_entry(
            master_password.to_owned(),
            record.name.unwrap_or("Untitled".to_owned()),
            record.description,
            record.username,
            record.password,
            record.url,
        )
        .await?;
    }
    Ok(())
}
