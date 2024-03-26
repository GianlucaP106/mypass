use model::entities::prelude::Entry;
use model::entities::*;
use sea_orm::IntoActiveModel;
use sea_orm::{self, prelude::Uuid, ActiveModelTrait, ActiveValue::Set, EntityTrait};
use url::Url;

use crate::master;
use crate::{
    crypto::{self},
    persistence,
};

pub async fn create_entry(
    master_password: String,
    name: String,
    description: Option<String>,
    username: Option<String>,
    password: String,
    url: Option<String>,
) -> Result<entry::Model, String> {
    let con = persistence::connect().await;
    let master = master::get_master()
        .await
        .expect("Master must be configured");

    let id = Uuid::new_v4().to_string();

    if let Some(u) = url.to_owned() {
        validate_url(u.to_owned())?;
    }

    let encrypted_password: Vec<u8> = crypto::encrypt_password(
        master_password.to_owned(),
        password.to_owned(),
        id.to_owned(),
        master.id.to_owned(),
    )?;

    let ent = entry::ActiveModel {
        id: Set(id),
        name: Set(name),
        description: Set(description),
        url: Set(url),
        username: Set(username),
        password: Set(encrypted_password),
    };
    ent.insert(&con).await.map_err(|e| e.to_string())
}

pub async fn modify_entry(
    entry_id: String,
    name: Option<String>,
    description: Option<String>,
    username: Option<String>,
    url: Option<String>,
) -> Result<entry::Model, String> {
    let con = persistence::connect().await;
    let mut entry: entry::ActiveModel = Entry::find_by_id(entry_id)
        .one(&con)
        .await
        .map_err(|_| "Error modifying entry")?
        .ok_or("Error modifying entry")?
        .into_active_model();

    if let Some(name) = name {
        entry.name = Set(name);
    }

    if let Some(description) = description {
        entry.description = Set(Some(description));
    }

    if let Some(username) = username {
        entry.username = Set(Some(username))
    }

    if let Some(url) = url {
        validate_url(url.to_owned())?;
        entry.url = Set(Some(url));
    }

    entry
        .update(&con)
        .await
        .map_err(|_| "Failed to update entry".to_owned())
}

pub async fn delete_entry(entry_id: String) -> Result<(), String> {
    let conn = persistence::connect().await;
    let entry = Entry::find_by_id(entry_id)
        .one(&conn)
        .await
        .map_err(|_| "Error deleting entry")?
        .ok_or("Error deleting entry")?
        .into_active_model();

    entry
        .delete(&conn)
        .await
        .map_err(|_| "Error delete entry".to_owned())
        .map(|_| ())
}

pub async fn get_entry(entry_id: String) -> Result<entry::Model, String> {
    let conn = persistence::connect().await;
    Entry::find_by_id(entry_id)
        .one(&conn)
        .await
        .map_err(|_| "Error fetching entry")?
        .ok_or("Error fetching entry".to_owned())
}

pub async fn get_all_entries() -> Result<Vec<entry::Model>, String> {
    let con = persistence::connect().await;
    Entry::find()
        .all(&con)
        .await
        .map_err(|_| "Error getting all entries".to_owned())
}

pub fn validate_url(url: String) -> Result<(), String> {
    Url::parse(url.as_ref()).map_err(|e| format!("Invalid URL: {}", e))?;
    Ok(())
}
