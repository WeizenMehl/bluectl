use clap::Parser;
mod lib;

///And CLI bluetooth manager
#[derive(Parser)]
struct  Cli{
    /// Status of the bluetooth adapter
    #[arg(short,long)]
    status: bool,
}

#[tokio::main]
async fn main() -> bluer::Result<()> {

    let bluetooth_context = lib::init().await?;

    let args = Cli::parse();
    match args{
        _status => lib::status(&bluetooth_context).await?,
    }
    return Ok(());
}
