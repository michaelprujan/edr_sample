use std::error::Error;

use clap::Parser;
mod edr;
use edr::process::process_edr;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "EDR Windows client application using QUIC.",
    long_about = None
)]
struct Args {
    /// Providers file (default: providers.json)
    #[arg(short = 'f', long, default_value = "providers.json")]
    providers_file: String,

    /// Server IP address
    #[arg(short = 'i', long, default_value = "127.0.0.1")]
    ip_address: String,

    /// Server port (default: 443 for QUIC protocol)
    #[arg(short = 'p', long, default_value_t = 443)]
    port: u16,

    /// Event collection time in seconds (default: 5 in sec)
    #[arg(short = 't', long, default_value_t = 5)]
    duration_in_sec: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Providers File Name: {}", args.providers_file);
    println!("IP Address: {}", args.ip_address);
    println!("Port: {}", args.port);
    println!("Running test time in seconds: {}", args.duration_in_sec);

    println!("IP Address: {}", args.ip_address);

    process_edr(
        args.providers_file.as_str(),
        args.ip_address,
        args.port,
        args.duration_in_sec,
    )
    .await?;

    Ok(())
}
