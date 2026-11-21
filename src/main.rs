use anyhow::Result;
use bluectl::{
    BluetoothContext, connect, disconnect, init, pair, remove_device, set_adapter, status,
    toggle_discoverable, toggle_discovery, toggle_pairable, toggle_powered,
};
use bluer::Address;
use clap::{CommandFactory, Parser, Subcommand};
use std::{env::args_os, path::PathBuf, time::Duration};
use tokio::sync::oneshot::{Sender, channel};
use reedline::{DefaultPrompt, Reedline, Signal, DefaultPromptSegment,ExternalPrinter};
use std::sync::mpsc::SyncSender;

mod output;
use output::{
    print_adapter_state_toggle, print_device_actions, print_discovery_stream,
    print_set_adapter_results, print_status,
};

use crate::output::print_discovery_stream_thread;

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
    ToggleDiscoveryStream,
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

    //reedline should replace rustyline
    let printer:ExternalPrinter<String> = ExternalPrinter::new(100);

    let printer_tx = printer.sender();

    let mut line_editor = Reedline::create()
        .with_external_printer(printer)
        .with_poll_interval(Duration::from_millis(100));

    let prompt = DefaultPrompt::new(
    DefaultPromptSegment::Basic("bluectl".into()),
    DefaultPromptSegment::Empty,
    );

    if args_os().len() > 1{
        let args = Cli::parse();
        call_commands(args, &mut bluetooth_context,&mut state_dir ,&mut sender, printer_tx).await?;
    }
    else{
        loop {
            let mut command: Vec<String> = vec!["bluectl".to_string()];

            let sig = line_editor.read_line(&prompt);

            match sig {
            Ok(Signal::Success(buffer)) => {
                command.extend(buffer.lines().map(String::from));
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
                            match call_commands(args, &mut bluetooth_context,&mut state_dir ,&mut sender, printer_tx.clone()).await{
                                Ok(None) => {}
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
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC(_)) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
            }
        }
    }
    Ok(())
}


pub async fn call_commands(args: Cli,mut bluetooth_context: &mut BluetoothContext,state_dir: &mut PathBuf ,sender: &mut Option<Sender<bool>>,printer_tx: SyncSender<String>) -> Result<Option<Sender<bool>>>{
    match args.command{
        Commands::Status => print_status(&status(&bluetooth_context).await?)?,
        Commands::SetAdapter {adapter_name} => print_set_adapter_results(&set_adapter(&mut bluetooth_context,state_dir, &adapter_name).await?)?,
        Commands::TogglePowered => print_adapter_state_toggle(&toggle_powered(&bluetooth_context).await?, "powered")?,
        Commands::ToggleDiscoverable => print_adapter_state_toggle(&toggle_discoverable(&bluetooth_context).await?, "discoverable")?,
        Commands::TogglePairable => print_adapter_state_toggle(&toggle_pairable(&bluetooth_context).await?, "pairable")?,
        Commands::ToggleDiscoveryStream => {
            match toggle_discovery(&bluetooth_context).await{
                Ok(Some(discovery_stream)) => {
                    let (tx, rx) = channel::<bool>();

                    if args_os().len() > 1{
                        print_discovery_stream(discovery_stream,&bluetooth_context.adapter).await?;
                    }

                    else{
                        let adapter = bluetooth_context.adapter.clone();

                        tokio::spawn(async move {
                            print_discovery_stream_thread(discovery_stream,&adapter,&printer_tx, rx).await.expect("Couldn't start discovery");
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