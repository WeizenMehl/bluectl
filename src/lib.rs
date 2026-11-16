use bluer::{Adapter, AdapterEvent, Address, Device, Session};
use anyhow::Result;
use futures::Stream;
use std::fs::{create_dir_all, read_to_string, remove_dir_all};
use std::path::{PathBuf};
use std::env::var_os;
use std::fs;

pub struct AppContext{
    pub bluetooth_context: BluetoothContext,
    pub state_dir: PathBuf,
}


pub struct BluetoothContext{
    pub adapter_names: Vec<String>,
    pub adapter: Adapter,
    pub session: Session,
}
impl BluetoothContext{
    pub async fn contains_device(&self,addr:Address) -> Result<bool>{
        let addresses = self.adapter.device_addresses().await?;
        Ok(addresses.contains(&addr))
    }
}

pub async fn init()-> Result<AppContext>{

    let session = Session::new().await?;
    let adapter_names = session.adapter_names().await?;


    let state_dir = get_state_dir()?;

    let file_path = state_dir.join("adapter.txt");
    let mut adapter = session.default_adapter().await?;

    if file_path.exists(){
        let value = read_to_string(file_path)?;
        adapter = session.adapter(value.trim())?;
    }

    let bluetooth_context = BluetoothContext{
        adapter: adapter,
        adapter_names:adapter_names,
        session: session,
    };


    let app_context = AppContext{
        bluetooth_context,
        state_dir,
    };
    return Ok(app_context)
}

pub enum SetAdapterResult{
    AlreadySet,
    DefaultSet,
    Changed,
    NotFound,
}

pub async fn set_adapter(bluetooth_context:&mut BluetoothContext,state_dir:&mut PathBuf ,searched_name:&str) -> Result<(SetAdapterResult, String)>{

    if searched_name == bluetooth_context.adapter.name(){
        return Ok((SetAdapterResult::AlreadySet,searched_name.to_string()));
    }
    else if searched_name == bluetooth_context.session.default_adapter().await?.name(){
        bluetooth_context.adapter = bluetooth_context.session.default_adapter().await?;
        let path = state_dir.clone();
        remove_dir_all(path)?;
        return Ok((SetAdapterResult::DefaultSet,searched_name.to_string()));
    }
    else{
        // In theory I could just try setting the adapter and avoid iterating above the hole list
        // But I wouldnt know which error I would get, so the function may wrongly return an 'not found' error even tho something diffrent went wrong
        // Also the adapter_names list wont be that long, so the performance loss isnt huge 
        match bluetooth_context.adapter_names.iter().find(|&x| x == searched_name){
            Some(adapter_name) => {
                create_dir_all(&state_dir)?;
                let path = state_dir.join("adapter.txt");
                fs::write(path, bluetooth_context.adapter.name())?;

                bluetooth_context.adapter = bluetooth_context.session.adapter(adapter_name)?;

                return Ok((SetAdapterResult::Changed,searched_name.to_string()));
            }
            None => return Ok((SetAdapterResult::NotFound,searched_name.to_string())),
        }
    }
}

pub fn get_state_dir() -> Result<PathBuf>{
    let base = var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(||{
            var_os("HOME")
                .map(|home| PathBuf::from(home)
                .join(".local/state"))
        })
        .ok_or_else(|| anyhow::anyhow!("Coudlnt find state directory"))?;
    Ok(base.join("bluectl"))
}

pub struct AdapterStatus{
    pub adapter_name: String,
    pub adapter_alias: String,
    pub adapter_adress: Address,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
}

pub async fn status(bluetooth_context:&BluetoothContext) -> Result<Vec<AdapterStatus>>{
    let mut adapters_states:Vec<AdapterStatus> = Vec::new();
    for adapter_name in &bluetooth_context.adapter_names{
        let adapter = bluetooth_context.session.adapter(&adapter_name)?;

        let adpater_status = AdapterStatus{
            adapter_name: adapter_name.clone(),
            adapter_alias: adapter.alias().await?,
            adapter_adress: adapter.address().await?,
            powered: adapter.is_powered().await?, 
            discoverable: adapter.is_discoverable().await?,
            pairable: adapter.is_pairable().await?,
        };

        adapters_states.push(adpater_status);
    }
    Ok(adapters_states)
}


pub enum ToggleAdapterState {
    On,
    Off,
}
pub async fn toggle_powered(bluetooth_context:&BluetoothContext) -> Result<ToggleAdapterState>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_powered().await? {
        adapter.set_powered(false).await?;
        Ok(ToggleAdapterState::Off)
    }
    else {
        adapter.set_powered(true).await?;
        Ok(ToggleAdapterState::On)
    }
}


pub async fn toggle_discoverable(bluetooth_context:&BluetoothContext) -> Result<ToggleAdapterState>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_discoverable().await? {
        adapter.set_discoverable(false).await?;
        Ok(ToggleAdapterState::Off)
    }
    else {
        adapter.set_discoverable(true).await?;
        Ok(ToggleAdapterState::On)
    }
}

pub async fn toggle_pairable(bluetooth_context:&BluetoothContext) -> Result<ToggleAdapterState>{

    let adapter = &bluetooth_context.adapter;
    if adapter.is_pairable().await? {
        adapter.set_pairable(false).await?;
        Ok(ToggleAdapterState::Off)
    }
    else {
        adapter.set_pairable(true).await?;
        Ok(ToggleAdapterState::On)
    }
}

pub async fn toggle_discovery(bluetooth_context:&BluetoothContext) -> Result<Option<impl Stream<Item = AdapterEvent> + Send + use<>>>{
    
    if !bluetooth_context.adapter.is_discovering().await? {    
        let discover = bluetooth_context.adapter.discover_devices().await?;
        Ok(Some(discover))
    }
    else{
        return Ok(None);
    }
}

pub enum DeviceActionResult{
    Success,
    AlreadyDone,
    NotConnected,
    DeviceNotFound,
    DeviceRemoved,
}

pub async fn pair(bluetooth_context:&BluetoothContext, addr:Address) -> Result<(DeviceActionResult, Device)>{
    let device = bluetooth_context.adapter.device(addr)?;
    if !bluetooth_context.contains_device(addr).await? {
        return Ok((DeviceActionResult::DeviceNotFound,device));
    }

    if device.is_paired().await? {
        return  Ok((DeviceActionResult::AlreadyDone,device));
    }

    device.pair().await?;
    Ok((DeviceActionResult::Success,device))
}

pub async fn connect(bluetooth_context:&BluetoothContext, addr:Address) -> Result<(DeviceActionResult, Device)>{
    let device = bluetooth_context.adapter.device(addr)?;
    if !bluetooth_context.contains_device(addr).await? {
        return Ok((DeviceActionResult::DeviceNotFound,device));
    }

    if !device.is_paired().await? {
        device.pair().await?;
    }

    if device.is_connected().await? {
        return Ok((DeviceActionResult::AlreadyDone,device));
    }

    device.connect().await?;
    Ok((DeviceActionResult::Success,device))
}

pub async fn disconnect(bluetooth_context:&BluetoothContext, addr:Address) -> Result<(DeviceActionResult, Device)>{
    let device = bluetooth_context.adapter.device(addr)?;
    if !bluetooth_context.contains_device(addr).await? {
        return Ok((DeviceActionResult::DeviceNotFound,device));
    }

    if !device.is_connected().await? {
        return Ok((DeviceActionResult::NotConnected,device));
    }

    device.disconnect().await?;
    Ok((DeviceActionResult::Success,device))
}

pub async fn remove_device(bluetooth_context:&BluetoothContext, addr:Address) -> Result<(DeviceActionResult, Device)>{
    let device = bluetooth_context.adapter.device(addr)?;
    if !bluetooth_context.contains_device(addr).await? {
        return Ok((DeviceActionResult::DeviceNotFound,device));
    }

    bluetooth_context.adapter.remove_device(addr).await?;
    Ok((DeviceActionResult::DeviceRemoved,device))
}
