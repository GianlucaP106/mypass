use clap::{Parser, Subcommand};

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
        username: Option<String>,

        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        url: Option<String>,
    },

    Config {
        #[command(subcommand)]
        commands: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ViewCommands {
    All {
        #[arg(short, long)]
        format: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    Master {
        #[command(subcommand)]
        commands: ConfigMasterCommands,
    },
}

#[derive(Subcommand)]
enum ConfigMasterCommands {
    Create {
        #[arg(short, long)]
        master_password: String,
    },
    View,
}

pub fn handle() {
    let cli = Cli::parse();
    match cli.command {
        RootCommands::View { commands } => match commands {
            ViewCommands::All { format } => {
                println!("Viewing");
            }
        },
        RootCommands::Create {
            name,
            username,
            password,
            url,
        } => {}
        RootCommands::Config { commands } => match commands {
            ConfigCommands::Master { commands } => match commands {
                ConfigMasterCommands::Create { master_password } => {
                    println!("config maser create")
                }
                ConfigMasterCommands::View => {
                    println!("config master view")
                }
            },
        },
    }
}
