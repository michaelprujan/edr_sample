use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) type Events = HashMap<u16, Vec<String>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EventsInfo {
    pub map: Events,
}
pub(crate) fn serialize(data: Arc<Mutex<Events>>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Safely access the data inside the Arc<Mutex<>>
    let data_guard = data.lock().unwrap();

    // Convert the HashMap into a serializable struct
    let serializable_data = EventsInfo {
        map: data_guard.clone(),
    };

    // Serialize the data using MessagePack
    let serialized = rmp_serde::to_vec(&serializable_data)?;

    Ok(serialized)
}
