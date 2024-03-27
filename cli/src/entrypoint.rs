use clap::{Parser, Subcommand};

use crate::{
    entries::{
        create_entry, delete_entry, export_entries, import_entries, update_entry, view_all_entries,
        view_entry,
    },
    master::{create_master, view_master},
};

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: RootCommands,
}

#[derive(Subcommand)]
enum RootCommands {
    /// View passwowrd entries
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

    /// Export entries as csv
    Export {
        /// Path to export (default is ~/.mypass/entries.csv)
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Import entries as csv
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
    All,

    /// View one password entry
    One {
        /// The number of the entry to view
        #[arg(short, long)]
        number: Option<usize>,

        /// Decrypt and reveal the password
        #[arg(short, long)]
        password: bool,

        /// Copy password to clipboard
        #[arg(short, long)]
        copy: bool,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
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
        } => match commands {
            Some(command) => match command {
                ViewCommands::All => {
                    view_all_entries().await.ok();
                }
                ViewCommands::One {
                    number,
                    password,
                    copy,
                } => {
                    view_entry(number, password, copy).await.ok();
                }
            },

            None => {
                if number.is_some() {
                    view_entry(number, password, copy).await.ok();
                } else if password || copy {
                    println!("You may only specify the password or copy option with the number option `-n`");
                    return;
                } else {
                    view_all_entries().await.ok();
                }
            }
        },
        RootCommands::Create {
            name,
            description,
            username,
            url,
        } => {
            create_entry(name, description, username, url).await.ok();
        }
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
            ConfigCommands::Master => {
                if api::master::is_master_configured().await {
                    view_master().await.ok();
                } else {
                    create_master().await.ok();
                }
            }
        },
    };
}

async fn enforce_configured_master(cli: &Cli) -> Result<(), ()> {
    // TODO: Btter way to do this
    if let RootCommands::Config {
        commands: ConfigCommands::Master,
    } = cli.command
    {
    } else if !api::master::is_master_configured().await {
        println!("Master is not configured. Please create a master key. (`mypass config master`)");
        return Err(());
    }
    Ok(())
}
