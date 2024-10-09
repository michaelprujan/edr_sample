use quinn::{Endpoint, RecvStream, SendStream};
use std::error::Error;
use std::net::SocketAddr;

pub(crate) async fn connect(
    server_addr: SocketAddr,
) -> Result<(SendStream, RecvStream), Box<dyn Error>> {
    // Create the client endpoint

    let endpoint = Endpoint::client("[::]:0".parse()?)?;

    // Connect to the server
    let connection = endpoint.connect(server_addr, "localhost")?;

    let (send, rcv) = connection.await?.open_bi().await?;

    // Return both send and receive streams
    Ok((send, rcv))
}
