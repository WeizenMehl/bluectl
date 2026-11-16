use anyhow::Result;
use bluectl::{
    BluetoothContext, connect, disconnect, init, pair, remove_device, set_adapter, status,
    toggle_discoverable, toggle_discovery, toggle_pairable, toggle_powered,
};
use bluer::Address;
use clap::{CommandFactory, Parser, Subcommand};
use rustyline::{Editor, history::FileHistory};
use shlex::split;
use std::{env::args_os, path::PathBuf};
use tokio::sync::oneshot::{Sender, channel};

mod output;
use output::{
    print_adapter_state_toggle, print_device_actions, print_discovery_stream,
    print_set_adapter_results, print_status,
};

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

    let app_context = init().await?;
    let mut bluetooth_context = app_context.bluetooth_context;
    let mut state_dir = app_context.state_dir;
    let mut sender: Option<Sender<bool>> = None;
    let mut rl = rustyline::DefaultEditor::new()?;

    if args_os().len() > 1{
        let args = Cli::parse();
        call_commands(args, &mut bluetooth_context,&mut state_dir ,&mut sender, &mut rl).await?;
    }
    else{
        loop {
                
            let readline = rl.readline("[bluectl]:")?;
            let mut command: Vec<String> = vec!["bluectl".to_string()];

            let mut arguments = split(&readline).unwrap();
                
            command.append(&mut arguments);

            match command[1].as_str(){
                "exit" => {
                    return Ok(());
                }
                "help" => {
                    let mut cmd = Cli::command();
                    let mut output = Vec::new();

                    cmd.write_help(&mut output)?;

                    println!("{}", String::from_utf8_lossy(&output));
                }

                &_ => {
                    match Cli::try_parse_from(command) {
                        Ok(args) => {
                            match call_commands(args, &mut bluetooth_context,&mut state_dir ,&mut sender, &mut rl).await{
                                Ok(tx) => sender = tx,
                                Err(err) => println!("{}",err),
                            }
                        }
                        Err(err) => {
                            println!("{}",err);
                        }
                    }

                }
            }
        }
    }
    Ok(())
}


pub async fn call_commands(args: Cli,mut bluetooth_context: &mut BluetoothContext,state_dir: &mut PathBuf ,sender: &mut Option<Sender<bool>>,rl : &mut Editor<(), FileHistory>) -> Result<Option<Sender<bool>>>{
    match args.command{
        Commands::Status => print_status(&status(&bluetooth_context).await?)?,
        Commands::SetAdapter {adapter_name} => print_set_adapter_results(&set_adapter(&mut bluetooth_context,state_dir, &adapter_name).await?)?,
        Commands::TogglePowered => print_adapter_state_toggle(&toggle_powered(&bluetooth_context).await?, "powered")?,
        Commands::ToggleDiscoverable => print_adapter_state_toggle(&toggle_discoverable(&bluetooth_context).await?, "discoverable")?,
        Commands::TogglePairable => print_adapter_state_toggle(&toggle_pairable(&bluetooth_context).await?, "pairable")?,
        Commands::ToggleDiscovery => {
            match toggle_discovery(&bluetooth_context).await{
                Ok(Some(discovery_stream)) => {
                    let (tx, rx) = channel::<bool>();

                    if args_os().len() > 1{
                        print_discovery_stream(discovery_stream,rl.create_external_printer()?,rx).await?;
                    }

                    else{
                        let printer = rl.create_external_printer()?;

                        tokio::spawn(async move {
                            print_discovery_stream(discovery_stream, printer, rx).await.expect("Couldn't start discovery");
                        }); 
                    }

                    return Ok(Some(tx));
                }
                Ok(None) => {
                    if let Some(tx) = sender.take() {
                        tx.send(true).unwrap();
                    }
                }
                Err(e) => println!("Error:{}", e), 
            }    
        },
        Commands::Pair {addr} => print_device_actions(&pair(&bluetooth_context, addr).await?, "paired").await?,
        Commands::Connect { addr } => print_device_actions(&connect(&bluetooth_context, addr).await?, "connected").await?,
        Commands::Disconnect { addr } => print_device_actions(&disconnect(&bluetooth_context, addr).await?, "disconnected").await?,
        Commands::RemoveDevice { addr } => print_device_actions(&remove_device(&bluetooth_context, addr).await?, "removed").await?,
        }
    Ok(None)
}