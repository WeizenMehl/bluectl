use bluer::{self, Adapter, Session};
use futures::StreamExt;
use std::thread;
use std::time::Duration;

pub struct BluetoothContext{
    pub adapter_names: Vec<String>,
    pub adapter: Adapter,
    pub session: Session,
}

pub async fn init()-> bluer::Result<BluetoothContext>{

    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;


    let bluetooth_context = BluetoothContext{
        adapter: session.default_adapter().await?,
        adapter_names:adapter_names,
        session: session,
    };
    return Ok(bluetooth_context)
}

// Has no use now and doesnt work anyway. Just wanted to have it already because its simple and I dont wanna forget
// Will find its use once I bring ratatui into the project
pub fn set_adapter(bluetooth_context:&mut BluetoothContext, searched_name:&str) -> bluer::Result<()>{

    match bluetooth_context.adapter_names.iter().find(|&x| x == searched_name){
        Some(adapter_name) => {
            bluetooth_context.adapter = bluetooth_context.session.adapter(adapter_name)?;
            println!("Adapter '{}' is now used for all commands", searched_name);
        }
        None => println!("Couldnt find adapter named {}",searched_name), 
    }
    Ok(())
}


pub async fn status(bluetooth_context:&BluetoothContext) -> bluer::Result<()>{
    for adapter_name in &bluetooth_context.adapter_names{
        let adapter = bluetooth_context.session.adapter(&adapter_name)?;
        println!("Name: {}", adapter_name);
        let alias = adapter.alias().await?;
        println!("\tAlias: {}", alias);
        let address = adapter.address().await?;
        println!("\tAddress: {}", address);
        let powered = adapter.is_powered().await?;
        println!("\tPowerd: {}", powered);
        let discoverable = adapter.is_discoverable().await?;
        println!("\tDiscoverable: {}", discoverable);
        let pairable = adapter.is_pairable().await?;
        println!("\tPairable: {}", pairable);
    }
    Ok(())
}

pub async fn toggle_powerd(bluetooth_context:&BluetoothContext) -> bluer::Result<()>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_powered().await? {
        adapter.set_powered(false).await?;
        println!("Adapter '{}' is now powerd off", adapter.name());
    }
    else {
        adapter.set_powered(true).await?;
        println!("Adapter '{}' is now powerd on", adapter.name());
    }

    Ok(())
}


pub async fn toggle_discoverable(bluetooth_context:&BluetoothContext) -> bluer::Result<()>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_discoverable().await? {
        adapter.set_discoverable(false).await?;
        println!("Adapter '{}' is now not discoverable", adapter.name());
    }
    else {
        adapter.set_discoverable(true).await?;
        println!("Adapter '{}' is now discoverable", adapter.name());
    }

    Ok(())
}

pub async fn toggle_pairable(bluetooth_context:&BluetoothContext) -> bluer::Result<()>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_pairable().await? {
        adapter.set_pairable(false).await?;
        println!("Adapter '{}' is now not pairable", adapter.name());
    }
    else {
        adapter.set_pairable(true).await?;
        println!("Adapter '{}' is now pairable", adapter.name());
    }

    Ok(())
}


// runs forever for now. But thats okay at least currently
pub async fn scan(bluetooth_context:&BluetoothContext)-> bluer::Result<()>{
    let mut discover = bluetooth_context.adapter.discover_devices().await?;
    
    while let Some(evt) = discover.next().await {
        println!("Discoverd: {:?}", evt);
    }

    Ok(())
}
