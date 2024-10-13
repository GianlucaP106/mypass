use crate::configuration;
use crate::crypto;
use crate::error::Error;
use model::entities::master;
use model::entities::prelude::Master;
use sea_orm::{self, prelude::Uuid, ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub async fn get_master() -> Result<Option<master::Model>, Error> {
    let conn = configuration::connect().await?;
    Master::find()
        .all(&conn)
        .await
        .map_err(|_| "Failed to get master".to_owned())
        .map(|entries| entries.first().map(|t| t.to_owned()))
}

pub async fn require_master() -> Result<master::Model, Error> {
    get_master()
        .await?
        .ok_or("Master not configured. Please create a master key".to_owned())
}

pub async fn create_master(password: String) -> Result<master::Model, Error> {
    if get_master().await?.is_some() {
        return Err("Master is already configured".to_owned());
    }
    let conn = configuration::connect().await?;
    let hashed_password = crypto::hash_password(password)?;
    let master = master::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set("Main Master (Default)".to_owned()),
        description: Set(Some("Master key to access your passwords".to_owned())),
        password: Set(hashed_password),
    };
    master.insert(&conn).await.map_err(|e| e.to_string())
}

pub async fn authenticate_master(master_password: String) -> Result<master::Model, Error> {
    let master = require_master().await?;
    crypto::verify_password(master_password, master.password.to_owned()).and_then(
        |is_authenticated| {
            if is_authenticated {
                Ok(master)
            } else {
                Err("Invalid master password".to_owned())
            }
        },
    )
}

pub async fn is_master_configured() -> Result<bool, Error> {
    Ok(get_master().await?.is_some())
}
