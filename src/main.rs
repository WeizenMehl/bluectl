use clap::{Parser,Subcommand};

///And CLI bluetooth manager
#[derive(Parser)]
#[command(name = "bluectl")]
struct  Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Status,
    TogglePowerd{
        adapter_name: String,
    },
    ToggleDiscoverable{
        adapter_name: String,
    },
    TogglePairable{
        adapter_name: String,
    }
}

#[tokio::main]
async fn main() -> bluer::Result<()> {

    let bluetooth_context = bluectl::init().await?;

    let args = Cli::parse();

    match args.command{
        Commands::Status => bluectl::status(&bluetooth_context).await?,
        Commands::TogglePowerd {adapter_name}=> bluectl::toggle_powerd(&bluetooth_context, &adapter_name).await?,
        Commands::ToggleDiscoverable {adapter_name}=> bluectl::toggle_discoverable(&bluetooth_context, &adapter_name).await?,
        Commands::TogglePairable {adapter_name}=> bluectl::toggle_pairable(&bluetooth_context, &adapter_name).await?,
    }
    return Ok(());
}
