use api::crypto;
use model::entities::entry;

use crate::{
    master::{prompt_authenticate, AuthenticatedMaster},
    util, view,
};

pub async fn view_all_entries() -> Result<(), ()> {
    let entries = api::entries::get_all_entries()
        .await
        .map_err(|e| println!("{}", e))?;
    view::print_entries(entries).map_err(|e| println!("{}", e))?;
    Ok(())
}

pub async fn view_entry(number: Option<usize>, view_pass: bool) -> Result<(), ()> {
    let number =
        util::unwrap_or_input_number(number, "Enter entry number: ", "Invalid entry number")?;
    let entry = entry_by_number(number).await?;
    let password = if view_pass {
        let master: AuthenticatedMaster = prompt_authenticate().await?;
        Some(
            crypto::decrypt_password(
                master.password,
                entry.password.to_owned(),
                entry.id.to_owned(),
                master.master.id,
            )
            .map_err(|e| println!("{}", e))?,
        )
    } else {
        None
    };
    view::print_entry(entry, number, password).map_err(|e| println!("{}", e))?;
    Ok(())
}

pub async fn entry_by_number(number: usize) -> Result<entry::Model, ()> {
    // TODO: put this number in the db
    let entries = api::entries::get_all_entries()
        .await
        .map_err(|e| println!("{}", e))?;

    if number == 0 || number > entries.len() {
        return Err(());
    }
    Ok(entries[number - 1].to_owned())
}

pub async fn create_entry(
    name: Option<String>,
    description: Option<String>,
    username: Option<String>,
    url: Option<String>,
) -> Result<(), ()> {
    let enter_a = |name: &str| -> String { format!("Enter a {} (skip to leave blank): ", name) };
    let name = util::unwrap_or_input(name, enter_a("name").as_ref());
    let description = util::unwrap_or_input(description, enter_a("description").as_ref());
    let url = util::unwrap_or_input(url, enter_a("url").as_ref());
    let password: String = util::get_password_with_prompt("Enter a password: ")?;
    let password2: String = util::get_password_with_prompt("Retype the password: ")?;
    if password != password2 {
        println!("Passwords must be the same");
        return Err(());
    }
    let master: AuthenticatedMaster = prompt_authenticate().await?;
    let entry = api::entries::create_entry(
        master.password,
        name.unwrap_or("Untitled".to_owned()),
        description,
        username,
        password,
        url,
    )
    .await
    .map_err(|e| println!("{}", e))?;
    view::print_entry(entry, 1, None).map_err(|e| println!("{}", e))?;
    Ok(())
}

pub async fn delete_entry(number: Option<usize>) -> Result<(), ()> {
    let number =
        util::unwrap_or_input_number(number, "Enter entry number: ", "Invalid entry number")?;
    let entry = entry_by_number(number).await?;
    prompt_authenticate().await?;

    api::entries::delete_entry(entry.id.to_owned())
        .await
        .map_err(|e| println!("{}", e))
}

pub async fn export_entries(path: Option<String>) -> Result<(), ()> {
    let path = util::unwrap_or_input(path, "Export path (default is ~/.mypass/entries.csv): ");
    let master = prompt_authenticate().await?;

    api::entry_transfer::export_entries(master.password, path)
        .await
        .map_err(|e| println!("{}", e))?;
    println!("Export finished");
    Ok(())
}

pub async fn import_entries(path: Option<String>) -> Result<(), ()> {
    let path = util::unwrap_or_input(path, "Import path (default is ~/.mypass/entries.csv): ");
    let master = prompt_authenticate().await?;
    api::entry_transfer::import_entries(master.password, path)
        .await
        .map_err(|e| println!("{}", e))?;
    println!("Import finished");
    Ok(())
}
