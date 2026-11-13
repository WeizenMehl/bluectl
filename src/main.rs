use bluectl::BluetoothContext;
use bluer::Address;
use clap::{Parser,Subcommand};
use anyhow::Result;
use rustyline::{Editor, history::FileHistory};
use shlex::self;
use tokio::sync::oneshot;

mod output;

/// A CLI Bluetooth manager
#[derive(Parser)]
#[command(name = "bluectl")]
pub struct  Cli{
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
    ToggleDiscovery,
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
    let mut sender: Option<tokio::sync::oneshot::Sender<bool>> = None;
    loop {
        let readline = rl.readline("[bluectl]:")?;
        let mut command: Vec<String> = vec!["bluectl".to_string()];

        let mut arguments = shlex::split(&readline).unwrap();

        if arguments[0] == "exit"{
            
            return Ok(())
        }
        
        command.append(&mut arguments);

        let args = Cli::parse_from(command);

        call_commands(args, &mut bluetooth_context, &mut sender, &mut rl).await?;
    }
}

pub async fn call_commands(args: Cli,mut bluetooth_context: &mut BluetoothContext,mut sender: &mut Option<tokio::sync::oneshot::Sender<bool>>, mut rl : &mut Editor<(), FileHistory>) -> Result<()>{
    match args.command{
        Commands::Status => output::print_status(&bluectl::status(&bluetooth_context).await?)?,
        Commands::SetAdapter {adapter_name} => bluectl::set_adapter(&mut bluetooth_context, &adapter_name)?,
        Commands::TogglePowered => output::print_adapter_state_toggle(&bluectl::toggle_powered(&bluetooth_context).await?, "powered")?,
        Commands::ToggleDiscoverable => output::print_adapter_state_toggle(&bluectl::toggle_discoverable(&bluetooth_context).await?, "discoverable")?,
        Commands::TogglePairable => output::print_adapter_state_toggle(&bluectl::toggle_pairable(&bluetooth_context).await?, "pairable")?,
        Commands::ToggleDiscovery => {
            match bluectl::toggle_discovery(&bluetooth_context).await{
                Ok(Some(discovery_stream)) => {
                    let (tx, rx) = oneshot::channel::<bool>();
                    sender = &mut Some(tx);
                    output::print_discovery_stream(discovery_stream, rx, &mut rl).await?;
                }
                Ok(None) => {
                    if let Some(tx) = sender.take() {
                        tx.send(true).unwrap();
                    }
                }
                Err(e) => println!("Error:{}", e), // I dont want any prints in main.rs but lets keep it for now
            }    
        },
        Commands::Pair {addr} => output::print_device_actions(&bluectl::pair(&bluetooth_context, addr).await?, "paired").await?,
        Commands::Connect { addr } => output::print_device_actions(&bluectl::connect(&bluetooth_context, addr).await?, "connected").await?,
        Commands::Disconnect { addr } => output::print_device_actions(&bluectl::disconnect(&bluetooth_context, addr).await?, "disconnected").await?,
        Commands::RemoveDevice { addr } => output::print_device_actions(&bluectl::remove_device(&bluetooth_context, addr).await?, "removed").await?,
        }
    Ok(())
}