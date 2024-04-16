use cli_table::{
    format::Justify, print_stdout, Cell, CellStruct, Color, Style, Table, TableStruct,
};
use model::entities::{entry, master};

pub fn print_entry(
    entry: entry::Model,
    number: usize,
    password: Option<String>,
    verbose: bool,
) -> Result<(), String> {
    let table = vec![format_entry(entry, number, password, verbose)]
        .table()
        .title(format_entry_title(verbose));
    print_stdout(format_table(table)).map_err(|_| "Error displaying")?;
    Ok(())
}

pub fn print_entries(entries: Vec<entry::Model>, verbose: bool) -> Result<(), String> {
    let empty = entries.is_empty();
    let table = entries
        .clone()
        .iter()
        .enumerate()
        .map(|(index, item)| format_entry(item.to_owned(), index + 1, None, verbose))
        .table()
        .title(format_entry_title(verbose));

    if empty {
        println!("No password entries. Create one with `mypass create`");
    } else {
        print_stdout(format_table(table)).map_err(|_| "Error displaying")?;
    }
    Ok(())
}

pub fn print_master(master: master::Model) -> Result<(), String> {
    let table = vec![format_master(master)]
        .table()
        .title(format_master_title());
    print_stdout(format_table(table)).map_err(|_| "Error displaying")?;
    Ok(())
}

pub fn print_path(master: master::Model, path: String) -> Result<(), String> {
    let mut title = format_master_title();
    title.push("Path to data store".to_owned().cell());
    let mut table = format_master(master);
    table.push(path.cell());
    let table = vec![table].table().title(title);
    print_stdout(format_table(table)).map_err(|_| "Error displaying")?;
    Ok(())
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
    if verbose {
        entry_row.push(entry.id.to_owned().cell());
        entry_row.push(entry.created_date.cell());
        entry_row.push(entry.modified_date.cell());
    }
    entry_row.push(entry.name.to_owned().cell());
    entry_row.push(
        entry
            .description
            .to_owned()
            .unwrap_or("None".to_owned())
            .cell(),
    );
    entry_row.push(entry.url.to_owned().unwrap_or("None".to_owned()).cell());
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
    if verbose {
        title.push("ID".to_owned().cell());
        title.push("Created Date".to_owned().cell());
        title.push("Last Modified Date".to_owned().cell());
    }
    title.push("Name".to_owned().cell());
    title.push("Description".to_owned().cell());
    title.push("URL".to_owned().cell());
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
