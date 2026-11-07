use std::ptr::addr_eq;

use bluer::Address;
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
    SetAdapter{
        adapter_name: String,
    },
    TogglePowerd,
    ToggleDiscoverable,
    TogglePairable,
    Scan,
    Pair{
        addr: Address,
    },
    Connect{
        addr: Address,
    },
}

#[tokio::main]
async fn main() -> bluer::Result<()> {

    let mut bluetooth_context = bluectl::init().await?;

    let args = Cli::parse();

    match args.command{
        Commands::Status => bluectl::status(&bluetooth_context).await?,
        Commands::SetAdapter {adapter_name}=> bluectl::set_adapter(&mut bluetooth_context, &adapter_name)?,
        Commands::TogglePowerd => bluectl::toggle_powerd(&bluetooth_context).await?,
        Commands::ToggleDiscoverable => bluectl::toggle_discoverable(&bluetooth_context).await?,
        Commands::TogglePairable => bluectl::toggle_pairable(&bluetooth_context).await?,
        Commands::Scan => bluectl::scan(&bluetooth_context).await?,
        Commands::Pair {addr} => bluectl::pair(&bluetooth_context, addr).await?,
        Commands::Connect { addr } => bluectl::connect(&bluetooth_context, addr).await?,
    }
    return Ok(());
}
