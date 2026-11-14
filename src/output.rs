use anyhow::Result;
use bluectl::{AdapterStatus, DeviceActionResult, ToggleAdapterState};
use bluer::{AdapterEvent, Device};
use futures::{Stream, StreamExt, pin_mut};
use rustyline::{Editor, ExternalPrinter, history::FileHistory};
use tokio::sync::oneshot::Receiver;

pub fn print_status(adapters_states: &Vec<AdapterStatus>)-> Result<()>{
    for state in adapters_states{
        println!("Name: {}", state.adapter_name);
        println!("\tAlias: {}", state.adapter_alias);
        println!("\tAddress: {}", state.adapter_adress);
        println!("\tPowered: {}", state.powered);
        println!("\tDiscoverable: {}", state.discoverable);
        println!("\tPairable: {}", state.pairable);
    }
    Ok(())
}

pub async  fn print_discovery_stream(discovery_stream:impl Stream<Item = AdapterEvent> + Send + 'static,mut recevier:Receiver<bool>,rl: &mut Editor<(), FileHistory>) -> Result<()>{
    let mut printer = rl.create_external_printer()?;
    tokio::spawn(async move {
        pin_mut!(discovery_stream);
        loop{
            tokio::select! {
                Some(evt) = discovery_stream.next() =>{
                    printer.print(format!("Discovered: {:?}", evt)).unwrap();
                }

                _ = &mut recevier =>{
                    println!("Stopped discovery");
                    break;
                }
            }
        }
    });
    Ok(())
}

pub fn print_adapter_state_toggle(state: &ToggleAdapterState,feature:&str) -> Result<()>{
    match state{
        ToggleAdapterState::On => println!("The Adapter is now {}",feature),
        ToggleAdapterState::Off => println!("The Adapter is now not {}", feature)
    }
    Ok(())
}

pub async fn print_device_actions(data:&(DeviceActionResult,Device), feature:&str) -> Result<()>{
    let (action,device) = data;
    match action{
        DeviceActionResult::Success => println!("Successfully {} following Device:\n\tName: {:?}\n\tAddress: {}",feature, device.name().await?,device.remote_address().await?),
        DeviceActionResult::AlreadyDone => println!("Following device is already {}:\n\tName: {:?}\n\tAddress: {}",feature, device.name().await?, device.remote_address().await?),
        DeviceActionResult::NotConnected => println!("Device is not connected:\nName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?),
        DeviceActionResult::DeviceNotFound => println!("Device is not known and needs to be discovered first."),
        DeviceActionResult::DeviceRemoved => println!("Removed the following device:\n\tAddress: {}", device.address()),
    }
    Ok(())
}