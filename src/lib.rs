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

pub async fn status(bluetooth_context :&BluetoothContext) -> bluer::Result<()>{
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
    }
    Ok(())
}