use model::entities::master;

use crate::{util, view};

pub struct AuthenticatedMaster {
    pub master: master::Model,
    pub password: String,
}

impl AuthenticatedMaster {
    pub fn new(master: master::Model, password: String) -> AuthenticatedMaster {
        AuthenticatedMaster { master, password }
    }
}

pub async fn authenticate(master_password: String) -> Result<master::Model, ()> {
    let master = api::master::authenticate_master(master_password)
        .await
        .map_err(|e| println!("{}", e))?;
    Ok(master)
}

pub async fn prompt_authenticate() -> Result<AuthenticatedMaster, ()> {
    let master_password = util::get_master_password()?;
    let master = authenticate(master_password.to_owned()).await?;
    Ok(AuthenticatedMaster::new(master, master_password))
}

pub async fn create_master() -> Result<(), ()> {
    let master_password = util::get_master_password()?;
    let master_password2 = util::get_password_with_prompt("Retype Master Password: ")?;
    if master_password != master_password2 {
        println!("Passwords are not the same, cancelling.");
        return Err(());
    }
    let master = api::master::create_master(master_password)
        .await
        .map_err(|e| println!("{}", e))?;
    view::print_master(master).map_err(|e| println!("{}", e))?;
    Ok(())
}

pub async fn view_master() -> Result<(), ()> {
    let master = prompt_authenticate().await?;
    view::print_master(master.master).map_err(|e| println!("{}", e))?;
    Ok(())
}

pub async fn view_path() -> Result<(), ()> {
    let master = api::master::get_master()
        .await
        .map_err(|_| println!("Failed to retrieve master"))?
        .unwrap();

    let path = api::persistence::get_path_to_db()
        .map_err(|_| println!("Failed to retrive path"))
        .unwrap();

    view::print_path(master, path).map_err(|e| println!("{}", e))?;
    Ok(())
}
