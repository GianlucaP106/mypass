use api::error::Error;
use cli_table::{
    format::Justify, print_stdout, Cell, CellStruct, Color, Style, Table, TableStruct,
};
use model::entities::{entry, master};

pub fn print_entry(
    entry: entry::Model,
    number: usize,
    password: Option<String>,
    verbose: bool,
) -> Result<(), Error> {
    let table = vec![format_entry(entry, number, password, verbose)]
        .table()
        .title(format_entry_title(verbose));
    print_table(table)
}

pub fn print_entries(entries: Vec<entry::Model>, verbose: bool) -> Result<(), Error> {
    if entries.is_empty() {
        println!("No password entries. Create one with `mypass create`");
        return Ok(());
    }
    let table = entries
        .clone()
        .iter()
        .enumerate()
        .map(|(index, item)| format_entry(item.to_owned(), index + 1, None, verbose))
        .table()
        .title(format_entry_title(verbose));

    print_table(table)?;
    Ok(())
}

pub fn print_master(master: master::Model) -> Result<(), Error> {
    let table = vec![format_master(master)]
        .table()
        .title(format_master_title());
    print_table(table)
}

pub fn print_path(path: String) -> Result<(), Error> {
    let table = vec![vec![path.cell()]]
        .table()
        .title(vec!["Path to data store".to_owned().cell()]);
    print_table(table)
}

fn format_entry(
    entry: entry::Model,
    number: usize,
    password: Option<String>,
    verbose: bool,
) -> Vec<CellStruct> {
    let color = if password.is_some() {
        Some(Color::Red)
    } else {
        Some(Color::White)
    };
    let mut entry_row: Vec<CellStruct> = Vec::new();
    entry_row.push(number.to_string().cell().justify(Justify::Center));
    entry_row.push(entry.name.to_owned().cell());
    if verbose {
        entry_row.push(entry.id.to_owned().cell());
        entry_row.push(entry.created_date.cell());
        entry_row.push(entry.modified_date.cell());
        entry_row.push(
            entry
                .description
                .to_owned()
                .unwrap_or("None".to_owned())
                .cell(),
        );
        entry_row.push(entry.url.to_owned().unwrap_or("None".to_owned()).cell());
    }
    entry_row.push(
        entry
            .username
            .to_owned()
            .unwrap_or("None".to_owned())
            .cell(),
    );
    entry_row.push(
        password
            .unwrap_or("**********".to_owned())
            .cell()
            .bold(true)
            .foreground_color(color),
    );
    entry_row
}

fn format_entry_title(verbose: bool) -> Vec<CellStruct> {
    let mut title: Vec<CellStruct> = Vec::new();
    title.push("Entry number".to_owned().cell());
    title.push("Name".to_owned().cell());
    if verbose {
        title.push("ID".to_owned().cell());
        title.push("Created Date".to_owned().cell());
        title.push("Last Modified Date".to_owned().cell());
        title.push("Description".to_owned().cell());
        title.push("URL".to_owned().cell());
    }
    title.push("Username".to_owned().cell());
    title.push("Password".to_owned().cell());
    title
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

fn format_table(table: TableStruct) -> TableStruct {
    table.foreground_color(Some(Color::Rgb(136, 192, 205)))
}

fn print_table(table: TableStruct) -> Result<(), Error> {
    print_stdout(format_table(table)).map_err(|_| "Error displaying".to_owned())
}
