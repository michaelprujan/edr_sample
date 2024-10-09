use ferrisetw::provider::Provider;
use ferrisetw::trace::UserTrace;
use ferrisetw::{EventRecord, SchemaLocator};
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::edr::event::{serialize, Events};
use crate::edr::io::read_json;
use crate::edr::parser::get_properties;
use crate::edr::quic_client::connect;

// Function to process EDR events, serialize them, and send via QUIC
pub async fn process_edr(
    providers_file: &str,
    server_ip: String,
    server_port: u16,
    duration_in_sec: u64,
) -> Result<(), Box<dyn Error>> {
    // Parse IP address and create a socket address
    let ip_addr = IpAddr::from_str(&server_ip)?;
    let server_addr: SocketAddr = SocketAddr::new(ip_addr, server_port);

    // Connect to the server via QUIC and get a SendStream
    let (mut send, _r) = connect(server_addr).await?;

    let events = read_events(providers_file, duration_in_sec)?;
    let buf = serialize(events)?;

    // Send the serialized buffer over the QUIC connection
    send.write_all(&buf).await?;

    Ok(())
}

fn read_events(
    providers_file: &str,
    duration_in_sec: u64,
) -> Result<Arc<Mutex<Events>>, Box<dyn std::error::Error>> {
    let guids = read_json(providers_file)?;
    let mut trace_builder = UserTrace::new();

    let events = Arc::new(Mutex::new(Events::new()));

    for guid in guids {
        let events_cloned = events.clone();
        trace_builder = trace_builder.enable(
            Provider::by_guid(&*guid)
                .add_callback(move |record, schema_locator| {
                    let _ = process_event(record, schema_locator, events_cloned.clone());
                })
                .build(),
        );
    }
    thread::sleep(Duration::from_secs(duration_in_sec));

    Ok(events)
}

fn process_event(
    record: &EventRecord,
    schema_locator: &SchemaLocator,
    events: Arc<Mutex<Events>>,
) -> Result<(), Box<dyn Error>> {
    // Attempt to get the event schema
    match schema_locator.event_schema(record) {
        Ok(_schema) => {
            /// Hack added as_raw_ptr_hack to get record raw pointer.
            let record_ptr = record.as_raw_ptr_hack();
            let properties = get_properties(
                record_ptr as *const windows::Win32::System::Diagnostics::Etw::EVENT_RECORD,
            )?;

            let mut events_locked = events.lock().unwrap();
            events_locked.insert(record.event_id(), properties);
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to get schema for event: {:?}", err);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get schema",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_events() {
        let file_path = r"C:\rust\cybereason\edr\config\1.json";
        let events = read_events(file_path, 2);
        assert!(events.is_ok())
    }
}
