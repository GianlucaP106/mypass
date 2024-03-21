use model::entities::prelude::Entry;
use model::entities::*;
use sea_orm::{self, prelude::Uuid, ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::persistence;

pub async fn create_entry(
    name: String,
    description: String,
    username: String,
    password: String,
) -> Result<entry::Model, String> {
    let con = persistence::connect().await;

    let ent = entry::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(name),
        description: Set(Some(description)),
        username: Set(Some(username)),
        password: Set(password),
    };
    match ent.insert(&con).await {
        Ok(e) => Ok(e),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn get_all_entries() -> Vec<entry::Model> {
    let con = persistence::connect().await;
    Entry::find()
        .all(&con)
        .await
        .expect("Failed to get all entries")
}
