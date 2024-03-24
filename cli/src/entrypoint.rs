use clap::{Parser, Subcommand};

use crate::{
    entries::{create_entry, delete_entry, view_all_entries, view_entry},
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
    View {
        #[command(subcommand)]
        commands: ViewCommands,
    },

    Create {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        username: Option<String>,

        #[arg(long)]
        url: Option<String>,
    },

    Delete {
        #[arg(short, long)]
        number: Option<usize>,
    },
    Config {
        #[command(subcommand)]
        commands: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ViewCommands {
    All,
    One {
        #[arg(short, long)]
        number: Option<usize>,

        #[arg(short, long)]
        view_password: bool,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    Master,
}

pub async fn run() {
    let cli = Cli::parse();

    if enforce_configured_master(&cli).await.is_err() {
        return;
    }

    match cli.command {
        RootCommands::View { commands } => match commands {
            ViewCommands::All => {
                view_all_entries().await.ok();
            }
            ViewCommands::One {
                number,
                view_password,
            } => {
                view_entry(number, view_password).await.ok();
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
    // TODO: Btter way to do this?
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
