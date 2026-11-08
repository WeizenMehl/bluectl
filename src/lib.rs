use bluer::{self, Adapter, Address, Session};
use futures::StreamExt;

pub struct BluetoothContext{
    pub adapter_names: Vec<String>,
    pub adapter: Adapter,
    pub session: Session,
}
impl BluetoothContext{
    pub async fn contains_device(&self,addr:Address) -> bluer::Result<bool>{
        let addresses = self.adapter.device_addresses().await?;

        if addresses.contains(&addr){
            return Ok(true);
        }
        println!("Device with address '{}' is not known and needs to be discovered first",addr);
        Ok(false)
    }
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

// Has no use now and doesn't work anyway. Just wanted to have it already because it's simple and I don't wanna forget
// Will find its use once I bring ratatui into the project
pub fn set_adapter(bluetooth_context:&mut BluetoothContext, searched_name:&str) -> bluer::Result<()>{

    match bluetooth_context.adapter_names.iter().find(|&x| x == searched_name){
        Some(adapter_name) => {
            bluetooth_context.adapter = bluetooth_context.session.adapter(adapter_name)?;
            println!("Adapter '{}' is now used for all commands", searched_name);
        }
        None => println!("Couldn't find adapter named {}",searched_name),
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
        println!("\tPowered: {}", powered);
        let discoverable = adapter.is_discoverable().await?;
        println!("\tDiscoverable: {}", discoverable);
        let pairable = adapter.is_pairable().await?;
        println!("\tPairable: {}", pairable);
    }
    Ok(())
}

pub async fn toggle_powered(bluetooth_context:&BluetoothContext) -> bluer::Result<()>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_powered().await? {
        adapter.set_powered(false).await?;
        println!("Adapter '{}' is now powered off", adapter.name());
    }
    else {
        adapter.set_powered(true).await?;
        println!("Adapter '{}' is now powered on", adapter.name());
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


// runs forever for now. But that's okay at least currently
pub async fn discovery(bluetooth_context:&BluetoothContext)-> bluer::Result<()>{
    let mut discover = bluetooth_context.adapter.discover_devices().await?;
    
    while let Some(evt) = discover.next().await {
        println!("Discovered: {:?}", evt);
    }

    Ok(())
}

pub async fn pair(bluetooth_context:&BluetoothContext, addr:Address) -> bluer::Result<()>{
    if !bluetooth_context.contains_device(addr).await? {
        return Ok(());
    }
    let device = bluetooth_context.adapter.device(addr)?;

    if device.is_paired().await? {
        println!("Following device is already paired:\n\tName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?);
        return Ok(());
    }

    device.pair().await?;
    println!("Paired with the following device:\n\tName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?);
    Ok(())
}

pub async fn connect(bluetooth_context:&BluetoothContext, addr:Address) -> bluer::Result<()>{
    if !bluetooth_context.contains_device(addr).await? {
        return Ok(());
    }
    let device = bluetooth_context.adapter.device(addr)?;

    if !device.is_paired().await? {
        device.pair().await?;
    }

    if device.is_connected().await? {
        println!("Following device is already connected:\n\tName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?);
        return Ok(());
    }

    device.connect().await?;
    println!("Connected with the following device:\n\tName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?);
    Ok(())
}

pub async fn disconnect(bluetooth_context:&BluetoothContext, addr:Address) -> bluer::Result<()>{
    if !bluetooth_context.contains_device(addr).await? {
        return Ok(());
    }
    let device = bluetooth_context.adapter.device(addr)?;

    if !device.is_connected().await? {
        println!("Device is not connected:\nName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?);
        return Ok(());
    }

    device.disconnect().await?;
    println!("Disconnected from the following device:\n\tName: {:?}\n\tAddress: {}", device.name().await?, device.remote_address().await?);
    Ok(())
}

pub async fn remove_device(bluetooth_context:&BluetoothContext, addr:Address) -> bluer::Result<()>{
    if !bluetooth_context.contains_device(addr).await? {
        return Ok(());
    }
    let device = bluetooth_context.adapter.device(addr)?;

    // Read name and address before removing, the device object is invalid afterwards
    let name = device.name().await?;
    let address = device.remote_address().await?;

    if !device.is_paired().await? {
        println!("Following device is not paired:\n\tName: {:?}\n\tAddress: {}", name, address);
        return Ok(());
    }

    bluetooth_context.adapter.remove_device(addr).await?;
    println!("Removed the following device:\n\tName: {:?}\n\tAddress: {}", name, address);
    Ok(())
}
