use clap::{Parser, Subcommand};

use crate::{
    entries::{
        create_entry, create_many, delete_entry, export_entries, import_entries, update_entry,
        view_all_entries, view_entry,
    },
    master::{create_master, move_db, set_path, view_master, view_path},
};

#[derive(Parser)]
#[command(version("0.4.1"), about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: RootCommands,
}

#[derive(Subcommand)]
enum RootCommands {
    /// View password entries
    View {
        /// The number of the entry to view
        #[arg(short, long)]
        number: Option<usize>,

        /// Decrypt and reveal the password
        #[arg(short, long)]
        password: bool,

        /// Copy password to clipboard
        #[arg(short, long)]
        copy: bool,

        /// Copy username to clipboard
        #[arg(long = "cusr")]
        copy_username: bool,

        /// Copy url to clipboard
        #[arg(long = "curl")]
        copy_url: bool,

        /// Display all columns
        #[arg(short, long)]
        verbose: bool,

        #[command(subcommand)]
        commands: Option<ViewCommands>,
    },

    /// Create a password entry
    Create {
        /// The name of the entry
        #[arg(short, long)]
        name: Option<String>,

        /// The description of the entry
        #[arg(short, long)]
        description: Option<String>,

        /// A username associated to the password
        #[arg(short, long)]
        username: Option<String>,

        /// A URL associated to the password
        #[arg(long)]
        url: Option<String>,

        #[command(subcommand)]
        commands: Option<CreateCommands>,
    },

    /// Update a password entry
    Update {
        /// The number of the entry to update
        #[arg(short, long)]
        number: Option<usize>,

        /// The name of the entry
        #[arg(long)]
        name: Option<String>,

        /// The description of the entry
        #[arg(short, long)]
        description: Option<String>,

        /// A username associated to the password
        #[arg(short, long)]
        username: Option<String>,

        /// A URL associated to the password
        #[arg(long)]
        url: Option<String>,
    },

    /// Delete a password entry
    Delete {
        /// The number of the entry to delete
        #[arg(short, long)]
        number: Option<usize>,
    },

    /// Export entries to csv
    Export {
        /// Path to export (default is ~/.mypass/entries.csv)
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Import entries from csv
    Import {
        /// Path to export (default is ~/.mypass/entries.csv)
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Configures MyPass
    Config {
        #[command(subcommand)]
        commands: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ViewCommands {
    /// View all password entries
    All {
        /// Display all columns
        #[arg(short, long)]
        verbose: bool,
    },

    /// View one password entry
    One {
        /// The number of the entry to view
        #[arg(short, long)]
        number: Option<usize>,

        /// Decrypt and reveal the password
        #[arg(short, long)]
        password: bool,

        /// Copy password, username or url to clipboard
        #[arg(short, long)]
        copy: bool,

        /// Copy username to clipboard
        #[arg(long = "cusr")]
        copy_username: bool,

        /// Copy url to clipboard
        #[arg(long = "curl")]
        copy_url: bool,

        /// Display all columns
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum CreateCommands {
    /// Create many entries interactively
    Many,
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// View Path to data store
    Path {
        /// Copy db file path to clipboard
        #[arg(short, long)]
        copy: bool,
    },

    /// Move the db file to a new path (will move the db for you)
    Move,

    /// Set the db file path
    Set {
        /// Path to set (assumes you manually moved the db to this path)
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Configure or view master
    Master,
}

pub async fn run() {
    let cli = Cli::parse();

    if enforce_configured_master(&cli).await.is_err() {
        return;
    }

    match cli.command {
        RootCommands::View {
            commands,
            number,
            password,
            copy,
            copy_username,
            copy_url,
            verbose,
        } => match commands {
            Some(command) => match command {
                ViewCommands::All { verbose } => {
                    view_all_entries(verbose).await.ok();
                }
                ViewCommands::One {
                    number,
                    password,
                    copy,
                    copy_username,
                    copy_url,
                    verbose,
                } => {
                    view_entry(number, password, copy, copy_username, copy_url, verbose)
                        .await
                        .ok();
                }
            },
            None => {
                if number.is_some() {
                    view_entry(number, password, copy, copy_username, copy_url, verbose)
                        .await
                        .ok();
                } else if password || copy {
                    println!("You may only specify the password or copy option with the number option `-n`");
                    return;
                } else {
                    view_all_entries(verbose).await.ok();
                }
            }
        },
        RootCommands::Create {
            name,
            description,
            username,
            url,
            commands,
        } => match commands {
            Some(command) => match command {
                CreateCommands::Many => {
                    create_many().await.ok();
                }
            },
            None => {
                create_entry(name, description, username, url).await.ok();
            }
        },
        RootCommands::Update {
            number,
            name,
            description,
            username,
            url,
        } => {
            update_entry(number, name, description, username, url)
                .await
                .ok();
        }
        RootCommands::Export { path } => {
            export_entries(path).await.ok();
        }
        RootCommands::Import { path } => {
            import_entries(path).await.ok();
        }
        RootCommands::Delete { number } => {
            delete_entry(number).await.ok();
        }
        RootCommands::Config { commands } => match commands {
            ConfigCommands::Path { copy } => {
                view_path(copy).await.ok();
            }
            ConfigCommands::Master => {
                let is_master_configured = api::master::is_master_configured()
                    .await
                    .map_err(|e| println!("{}", e))
                    .ok();
                if let Some(is_master_configured) = is_master_configured {
                    if is_master_configured {
                        view_master().await.ok();
                    } else {
                        create_master().await.ok();
                    }
                }
            }
            ConfigCommands::Move => {
                move_db().await.ok();
            }
            ConfigCommands::Set { path } => {
                set_path(path).await.ok();
            }
        },
    };
}

async fn enforce_configured_master(cli: &Cli) -> Result<(), ()> {
    let is_master_configured = api::master::is_master_configured()
        .await
        .map_err(|e| println!("{}", e))?;
    // TODO: Btter way to do this
    if let RootCommands::Config {
        commands: ConfigCommands::Master,
    } = cli.command
    {
    } else if !is_master_configured {
        println!("Master is not configured. Please create a master key. (`mypass config master`)");
        return Err(());
    }
    Ok(())
}
