use clap::{Parser,Subcommand};
mod lib;

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
    }
}

#[tokio::main]
async fn main() -> bluer::Result<()> {

    let bluetooth_context = lib::init().await?;

    let args = Cli::parse();

    match args.command{
        Commands::Status => lib::status(&bluetooth_context).await?,
        Commands::TogglePowerd {adapter_name}=> lib::toggle_powerd(&bluetooth_context, &adapter_name).await?,
        Commands::ToggleDiscoverable {adapter_name}=> lib::toggle_discoverable(&bluetooth_context, &adapter_name).await?,
    }
    return Ok(());
}
