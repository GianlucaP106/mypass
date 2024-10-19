use model::entities::master::{self};

use crate::{
    util::{self, PrintError},
    view,
};

#[derive(Clone)]
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
        .print_err()?;
    Ok(master)
}

pub async fn prompt_authenticate() -> Result<AuthenticatedMaster, ()> {
    let master_password = util::get_master_password()?;
    let master = authenticate(master_password.to_owned()).await?;
    Ok(AuthenticatedMaster::new(master, master_password))
}

pub async fn create_master() -> Result<(), ()> {
    let master_password = util::get_master_password()?;
    let master_password2 = util::get_password_with_prompt_print("Retype Master Password: ")?;
    if master_password != master_password2 {
        println!("Passwords are not the same, cancelling.");
        return Err(());
    }
    let master = api::master::create_master(master_password)
        .await
        .print_err()?;
    view::print_master(master).print_err()
}

pub async fn view_master() -> Result<(), ()> {
    let master = prompt_authenticate().await?;
    view::print_master(master.master).print_err()
}

pub async fn view_path(copy: bool) -> Result<(), ()> {
    let path = api::configuration::get_db_path().print_err()?;
    let path = path.to_string_lossy().into_owned();
    if copy {
        util::copy_to_clipboard(path.clone()).print_err()?;
    }
    view::print_path(path).print_err()
}

pub async fn move_db() -> Result<(), ()> {
    let new_path = util::input("Enter new db file path: ")
        .ok_or(())
        .map_err(|_| println!("File path is required"))?;
    api::configuration::move_db(new_path.clone()).print_err()
}

pub async fn set_path(path: Option<String>) -> Result<(), ()> {
    let path = path
        .or_else(|| util::input("New DB path: "))
        .ok_or("New DB path is required".to_string())
        .print_err()?;

    api::configuration::set_db_path(path).print_err()
}
