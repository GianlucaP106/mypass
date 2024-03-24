use crate::crypto;
use crate::persistence;
use model::entities::master;
use model::entities::prelude::Master;
use sea_orm::{self, prelude::Uuid, ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub async fn get_master() -> Option<master::Model> {
    let conn = persistence::connect().await;
    Master::find()
        .all(&conn)
        .await
        .map_err(|e| panic!("{}", e))
        .map(|entries| entries.first().map(|t| t.to_owned()))
        .ok()?

    // The above using match
    // match Master::find().all(&conn).await {
    //     Ok(master_entries) => master_entries.first().map(|first| first.to_owned()),
    //     Err(e) => {
    //         panic!("{e}")
    //     }
    // }
}

pub async fn create_master(password: String) -> Result<master::Model, String> {
    if get_master().await.is_some() {
        return Err("Master already configured".to_owned());
    }
    let conn = persistence::connect().await;
    let hashed_password = crypto::hash_password(password)?;
    let master = master::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set("Main".to_owned()),
        description: Set(Some("Main".to_owned())),
        password: Set(hashed_password),
    };
    master.insert(&conn).await.map_err(|e| e.to_string())
}

pub async fn authenticate_master(master_password: String) -> Result<master::Model, String> {
    let master = match get_master().await {
        Some(master) => master,
        None => return Err("Master not configured. Please create a master key".to_owned()),
    };
    if crypto::verify_password(master_password, master.password.to_owned())? {
        Ok(master)
    } else {
        Err("Invalid master password".to_owned())
    }
}

pub async fn is_master_configured() -> bool {
    get_master().await.is_some()
}
