use model::entities::prelude::Entry;
use model::entities::*;
use sea_orm::IntoActiveModel;
use sea_orm::{self, prelude::Uuid, ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::error::Error;
use crate::{
    crypto::{self},
    persistence,
};
use crate::{master, util};

pub async fn create_entry(
    master_password: String,
    name: String,
    description: Option<String>,
    username: Option<String>,
    password: String,
    url: Option<String>,
) -> Result<entry::Model, Error> {
    let con = persistence::connect().await?;
    let master = master::require_master().await?;
    let id = Uuid::new_v4().to_string();

    if let Some(u) = url.to_owned() {
        util::validate_url(u.to_owned())?;
    }

    let encrypted_password: Vec<u8> = crypto::encrypt_password(
        master_password.to_owned(),
        password.to_owned(),
        id.to_owned(),
        master.id.to_owned(),
    )?;

    let created_date = util::now();
    let modified_date = created_date.to_owned();

    let ent = entry::ActiveModel {
        id: Set(id),
        name: Set(name),
        description: Set(description),
        url: Set(url),
        username: Set(username),
        password: Set(encrypted_password),
        created_date: Set(created_date),
        modified_date: Set(modified_date),
    };
    ent.insert(&con)
        .await
        .map_err(|_| "Failed to create a new entry".to_owned())
}

pub async fn update_entry(
    entry_id: String,
    name: Option<String>,
    description: Option<String>,
    username: Option<String>,
    url: Option<String>,
    passwords: Option<(String, String)>,
) -> Result<entry::Model, Error> {
    let con = persistence::connect().await?;
    let err = "Error modifying entry";
    let mut entry: entry::ActiveModel = Entry::find_by_id(entry_id.to_owned())
        .one(&con)
        .await
        .map_err(|_| err)?
        .ok_or(err)?
        .into_active_model();

    let mut is_modified = false;

    if let Some(name) = name {
        entry.name = Set(name);
        is_modified = true;
    }

    if let Some(description) = description {
        entry.description = Set(Some(description));
        is_modified = true;
    }

    if let Some(username) = username {
        entry.username = Set(Some(username));
        is_modified = true;
    }

    if let Some(url) = url {
        util::validate_url(url.to_owned())?;
        entry.url = Set(Some(url));
        is_modified = true;
    }

    if let Some(passwords) = passwords {
        let (master_password, new_password) = passwords;
        let master = master::require_master().await?;
        let new_encrypted_password: Vec<u8> = crypto::encrypt_password(
            master_password.to_owned(),
            new_password.to_owned(),
            entry_id,
            master.id.to_owned(),
        )?;
        entry.password = Set(new_encrypted_password)
    }

    if is_modified {
        let modified_date = util::now();
        entry.modified_date = Set(modified_date);
    }

    entry
        .update(&con)
        .await
        .map_err(|_| "Failed to update entry".to_owned())
}

pub async fn delete_entry(entry_id: String) -> Result<(), Error> {
    let conn = persistence::connect().await?;
    let err = "Failed to delete entry";
    let entry = Entry::find_by_id(entry_id)
        .one(&conn)
        .await
        .map_err(|_| err)?
        .ok_or(err)?
        .into_active_model();

    entry
        .delete(&conn)
        .await
        .map_err(|_| err.to_owned())
        .map(|_| ())
}

pub async fn get_entry(entry_id: String) -> Result<entry::Model, Error> {
    let conn = persistence::connect().await?;
    let err = "Failed to fetch entry";
    Entry::find_by_id(entry_id)
        .one(&conn)
        .await
        .map_err(|_| err)?
        .ok_or(err.to_owned())
}

pub async fn get_all_entries() -> Result<Vec<entry::Model>, Error> {
    let con = persistence::connect().await?;
    Entry::find()
        .all(&con)
        .await
        .map_err(|_| "Failed to get all entries".to_owned())
}
