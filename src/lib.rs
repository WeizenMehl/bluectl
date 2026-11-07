use bluer::{self, Session};

pub struct BluetoothContext{
    pub session: Session,
    pub adapter_names: Vec<String>,
}

pub async fn init()-> bluer::Result<BluetoothContext>{

    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;


    let bluetooth_context = BluetoothContext{
        session: session,
        adapter_names:adapter_names,
    };
    return Ok(bluetooth_context)
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

pub async fn toggle_powerd(bluetooth_context:&BluetoothContext, searched_name:&String) -> bluer::Result<()>{

    match bluetooth_context.adapter_names.iter().find(|&x| x == searched_name){
        Some(adapter_name) => {
                let adapter = bluetooth_context.session.adapter(&adapter_name)?;
                if adapter.is_powered().await? {
                    adapter.set_powered(false).await?;
                    println!("Adapter {} is now powerd off", adapter_name);
                }
                else {
                    adapter.set_powered(true).await?;
                println!("Adapter {} is now powerd on", adapter_name);
                }
        }
        None => println!("No adapter named {}", searched_name)
    }

    Ok(())
}


pub async fn toggle_discoverable(bluetooth_context:&BluetoothContext, searched_name:&String) -> bluer::Result<()>{

    match bluetooth_context.adapter_names.iter().find(|&x| x == searched_name){
        Some(adapter_name) => {
                let adapter = bluetooth_context.session.adapter(&adapter_name)?;
                if adapter.is_discoverable().await? {
                    adapter.set_discoverable(false).await?;
                    println!("Adapter {} is now not discoverable", adapter_name);
                }
                else {
                    adapter.set_discoverable(true).await?;
                    println!("Adapter {} is now discoverable", adapter_name);
                }
        }
        None => println!("No adapter named {}", searched_name)
    }

    Ok(())
}

pub async fn toggle_pairable(bluetooth_context:&BluetoothContext, searched_name:&String) -> bluer::Result<()>{

    match bluetooth_context.adapter_names.iter().find(|&x| x == searched_name){
        Some(adapter_name) => {
                let adapter = bluetooth_context.session.adapter(&adapter_name)?;
                if adapter.is_pairable().await? {
                    adapter.set_pairable(false).await?;
                    println!("Adapter {} is now not pairable", adapter_name);
                }
                else {
                    adapter.set_pairable(true).await?;
                    println!("Adapter {} is now pairable", adapter_name);
                }
        }
        None => println!("No adapter named {}", searched_name)
    }

    Ok(())
}