use api::crypto;
use model::entities::entry;

use crate::{
    master::{prompt_authenticate, AuthenticatedMaster},
    util::{self, get_input_required, PrintError},
    view,
};

pub async fn view_all_entries(verbose: bool) -> Result<(), ()> {
    let entries = api::entries::get_all_entries().await.print_err()?;
    view::print_entries(entries, verbose).print_err()
}

pub async fn view_entry(
    number: Option<usize>,
    view_pass: bool,
    copy_password: bool,
    copy_username: bool,
    copy_url: bool,
    verbose: bool,
) -> Result<(), ()> {
    let number =
        util::unwrap_or_input_number(number, "Enter entry number: ", "Invalid entry number")?;
    let entry = entry_by_number(number).await?;

    let decrypted_password = if view_pass || copy_password {
        let master: AuthenticatedMaster = prompt_authenticate().await?;
        Some(
            crypto::decrypt_password(
                master.password,
                entry.password.to_owned(),
                entry.id.to_owned(),
                master.master.id,
            )
            .print_err()?,
        )
    } else {
        None
    };

    let item_to_copy: Option<String> = if copy_password {
        if copy_username || copy_url {
            println!("Only copying password");
        }
        decrypted_password.to_owned()
    } else if copy_username {
        if copy_url {
            println!("Only copying username");
        }
        entry.username.to_owned()
    } else if copy_url {
        entry.url.to_owned()
    } else {
        None
    };

    item_to_copy.map(util::copy_to_clipboard);

    let decrypted_password = if view_pass { decrypted_password } else { None };
    view::print_entry(entry, number, decrypted_password, verbose).print_err()?;
    Ok(())
}

pub async fn entry_by_number(number: usize) -> Result<entry::Model, ()> {
    let entries = api::entries::get_all_entries().await.print_err()?;
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
    let username = util::unwrap_or_input(username, enter_a("username").as_ref());
    let url = util::unwrap_or_input(url, enter_a("url").as_ref());
    let password: String =
        util::get_password_with_prompt("Enter a password (leave empty to generate): ")
            .unwrap_or_else(|_| api::crypto::generate_password());
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
    .print_err()?;
    view::print_entry(entry, 1, None, true).print_err()?;
    Ok(())
}

pub async fn create_many() -> Result<(), ()> {
    let mut m: Option<AuthenticatedMaster> = None;
    loop {
        println!("\n");
        println!("Control-c to stop");
        println!("\n");
        let enter_a =
            |name: &str| -> String { format!("Enter a {} (skip to leave blank): ", name) };
        let name = util::get_input_required(&enter_a("name"));
        let description = util::get_input_required(&enter_a("description"));
        let username = get_input_required(&enter_a("username"));
        let url = util::get_input_required(&enter_a("url"));
        let password: String = match util::get_password_with_prompt_print("Enter a password: ") {
            Ok(p) => p,
            Err(_) => continue,
        };
        let master: AuthenticatedMaster = if let Some(m) = m {
            m
        } else {
            prompt_authenticate().await?
        };
        m = Some(master.clone());
        let entry = api::entries::create_entry(
            master.password,
            name.unwrap_or("Untitled".to_owned()),
            description,
            username,
            password,
            url,
        )
        .await
        .print_err()?;
        view::print_entry(entry, 1, None, true).print_err()?;
    }
}

pub async fn update_entry(
    entry_number: Option<usize>,
    name: Option<String>,
    description: Option<String>,
    username: Option<String>,
    url: Option<String>,
) -> Result<(), ()> {
    let number =
        util::unwrap_or_input_number(entry_number, "Enter entry number: ", "Invalid entry number")?;
    let entry = entry_by_number(number).await?;
    let enter_a = |name: &str| -> String { format!("Enter a {} (skip to leave blank): ", name) };
    let name = util::unwrap_or_input(name, enter_a("name").as_ref());
    let description = util::unwrap_or_input(description, enter_a("description").as_ref());
    let username = util::unwrap_or_input(username, enter_a("username").as_ref());
    let url = util::unwrap_or_input(url, enter_a("url").as_ref());
    let password = rpassword::prompt_password("Enter a password (skip to leave blank): ")
        .map_err(|_| println!("Failed to get password"))
        .ok()
        .and_then(|p| if p.trim().is_empty() { None } else { Some(p) });
    let password: Option<String> = if let Some(p) = password {
        let retyped = util::get_password_with_prompt_print("Retype new password: ")?;
        if retyped != p {
            println!("Passwords must be the same");
            return Err(());
        }
        Some(p)
    } else {
        None
    };
    let passwords: Option<(String, String)> = if let Some(p) = password {
        let master: AuthenticatedMaster = prompt_authenticate().await?;
        Some((master.password, p))
    } else {
        None
    };
    let entry = api::entries::update_entry(entry.id, name, description, username, url, passwords)
        .await
        .print_err()?;
    view::print_entry(entry, number, None, true).print_err()?;
    Ok(())
}

pub async fn delete_entry(number: Option<usize>) -> Result<(), ()> {
    let number =
        util::unwrap_or_input_number(number, "Enter entry number: ", "Invalid entry number")?;
    let entry = entry_by_number(number).await?;
    prompt_authenticate().await?;

    api::entries::delete_entry(entry.id.to_owned())
        .await
        .print_err()
}

pub async fn export_entries(path: Option<String>) -> Result<(), ()> {
    let path = util::unwrap_or_input(path, "Export path (default is ~/.mypass/entries.csv): ");
    let master = prompt_authenticate().await?;

    api::entry_transfer::export_entries(master.password, path)
        .await
        .print_err()?;
    println!("Export finished");
    Ok(())
}

pub async fn import_entries(path: Option<String>) -> Result<(), ()> {
    let path = util::unwrap_or_input(path, "Import path (default is ~/.mypass/entries.csv): ");
    let master = prompt_authenticate().await?;
    api::entry_transfer::import_entries(master.password, path)
        .await
        .print_err()?;
    println!("Import finished");
    Ok(())
}
