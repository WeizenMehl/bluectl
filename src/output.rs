use futures::{StreamExt, pin_mut};
use rustyline::{Editor, ExternalPrinter, history::FileHistory};
use anyhow::Result;
use bluer::AdapterEvent;

pub fn print_status(adapters_states: Vec<bluectl::AdapterStatus>)-> Result<()>{
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

pub async  fn print_discovery_stream(discovery_stream:impl futures::Stream<Item = AdapterEvent> + Send + 'static,mut recevier:tokio::sync::oneshot::Receiver<bool>,rl: &mut Editor<(), FileHistory>) -> Result<()>{
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