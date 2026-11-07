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
    }
    return Ok(());
}
