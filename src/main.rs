use bluer::Address;
use clap::{Parser,Subcommand};
use anyhow::Result;
use shlex::self;

/// A CLI Bluetooth manager
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
    TogglePowered,
    ToggleDiscoverable,
    TogglePairable,
    Discovery,
    Pair{
        addr: Address,
    },
    Connect{
        addr: Address,
    },
    Disconnect{
        addr: Address,
    },
    RemoveDevice{
        addr: Address,
    },
}

#[tokio::main]
async fn main() -> Result<()> {

    let mut bluetooth_context = bluectl::init().await?;

    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline("[bluectl]")?;
        let mut command: Vec<String> = vec!["bluectl".to_string()];

        let mut arguments = shlex::split(&readline).unwrap();
        command.append(&mut arguments);

        let args = Cli::parse_from(command);

        match args.command{
            Commands::Status => bluectl::status(&bluetooth_context).await?,
            Commands::SetAdapter {adapter_name}=> bluectl::set_adapter(&mut bluetooth_context, &adapter_name)?,
            Commands::TogglePowered => bluectl::toggle_powered(&bluetooth_context).await?,
            Commands::ToggleDiscoverable => bluectl::toggle_discoverable(&bluetooth_context).await?,
            Commands::TogglePairable => bluectl::toggle_pairable(&bluetooth_context).await?,
            Commands::Discovery => bluectl::discovery(&bluetooth_context).await?,
            Commands::Pair {addr} => bluectl::pair(&bluetooth_context, addr).await?,
            Commands::Connect { addr } => bluectl::connect(&bluetooth_context, addr).await?,
            Commands::Disconnect { addr } => bluectl::disconnect(&bluetooth_context, addr).await?,
            Commands::RemoveDevice { addr } => bluectl::remove_device(&bluetooth_context, addr).await?,
        }
        return Ok(()) // will be removed later I am just to tired to implement an exit funciton
    }
}
