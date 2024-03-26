use cli_table::{
    format::Justify, print_stdout, Cell, CellStruct, Color, Style, Table, TableStruct,
};
use model::entities::{entry, master};

pub fn print_entry(
    entry: entry::Model,
    number: usize,
    password: Option<String>,
) -> Result<(), String> {
    let table = vec![format_entry(entry, number, password)]
        .table()
        .title(format_entry_title());
    print_stdout(formate_table(table)).map_err(|_| "Error displaying")?;
    Ok(())
}

pub fn print_entries(entries: Vec<entry::Model>) -> Result<(), String> {
    let empty = entries.is_empty();
    let table = entries
        .clone()
        .iter()
        .enumerate()
        .map(|(index, item)| format_entry(item.to_owned(), index + 1, None))
        .table()
        .title(format_entry_title());

    if empty {
        println!("No password entries. Create one with `mypass create`");
    } else {
        print_stdout(formate_table(table)).map_err(|_| "Error displaying")?;
    }
    Ok(())
}

pub fn print_master(master: master::Model) -> Result<(), String> {
    let table = vec![format_master(master)]
        .table()
        .title(format_master_title());
    print_stdout(formate_table(table)).map_err(|_| "Error displaying")?;
    Ok(())
}

fn format_entry(entry: entry::Model, number: usize, password: Option<String>) -> Vec<CellStruct> {
    let color = if password.is_some() {
        Some(Color::Red)
    } else {
        Some(Color::White)
    };
    vec![
        number.to_string().cell().justify(Justify::Center),
        entry.id.to_owned().cell(),
        entry.name.to_owned().cell(),
        entry
            .description
            .to_owned()
            .unwrap_or("None".to_owned())
            .cell(),
        entry.url.to_owned().unwrap_or("None".to_owned()).cell(),
        password
            .unwrap_or("**********".to_owned())
            .cell()
            .bold(true)
            .foreground_color(color),
    ]
}

fn format_entry_title() -> Vec<CellStruct> {
    vec![
        "Entry number".to_owned().cell(),
        "ID".to_owned().cell(),
        "Name".to_owned().cell(),
        "Description".to_owned().cell(),
        "Url".to_owned().cell(),
        "Password".to_owned().cell(),
    ]
}

fn format_master(master: master::Model) -> Vec<CellStruct> {
    vec![
        master.name.to_owned().cell(),
        master
            .description
            .to_owned()
            .unwrap_or("No description".to_owned())
            .cell(),
    ]
}

fn format_master_title() -> Vec<CellStruct> {
    vec!["Name".to_owned().cell(), "Description".to_owned().cell()]
}

fn formate_table(table: TableStruct) -> TableStruct {
    table.foreground_color(Some(Color::Rgb(136, 192, 205)))
}
