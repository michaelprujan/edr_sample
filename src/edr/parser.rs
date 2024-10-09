use windows::Win32::System::Diagnostics::Etw::EVENT_RECORD;
use windows::Win32::System::Diagnostics::Etw::*;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const ERROR_SUCCESS: u32 = 0;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct EventPropertyInfo {
    pub flags: u32,                 // Property flags
    pub name_offset: u32,           // Offset to the property name in the buffer
    pub non_struct_type: u32,       // Type of the property (non-struct)
    pub struct_start_index: u32,    // Index for the start of a struct (if the property is a struct)
    pub count_property_index: u32,  // Index of the count property
    pub length_property_index: u32, // Index of the length property
    pub reserved: u32,              // Reserved field
    pub length: u32,                // Length of the property
}

pub(crate) fn get_properties(
    event_record: *const EVENT_RECORD,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut buffer_size = 0;
    let mut status;

    unsafe {
        status = TdhGetEventInformation(
            event_record,
            None,
            Some(std::ptr::null_mut()),
            &mut buffer_size,
        );
    }

    if status != ERROR_INSUFFICIENT_BUFFER {
        return Err(format!("TdhGetEventInformation failed: status = {}", status).into());
    }

    // Step 2: Allocate a buffer for TRACE_EVENT_INFO using the buffer_size returned
    let mut buffer: Vec<u8> = vec![0u8; buffer_size as usize];

    // Step 3: Call TdhGetEventInformation again to fill the buffer
    status = unsafe {
        TdhGetEventInformation(
            event_record,
            None,
            Some(buffer.as_mut_ptr() as *mut TRACE_EVENT_INFO),
            &mut buffer_size,
        )
    };

    if status != ERROR_SUCCESS {
        return Err(format!(
            "TdhGetEventInformation failed on second call: status = {}",
            status
        )
        .into());
    }

    // Step 4: Parse the buffer into EventPropertyInfo
    let property_infos = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const EventPropertyInfo,
            buffer_size as usize / std::mem::size_of::<EventPropertyInfo>(),
        )
    };

    let mut properties = Vec::new();

    // Step 5: Iterate through each property and extract property names
    for (i, property_info) in property_infos.iter().enumerate() {
        // Debugging: Print property offset and length for investigation
        println!(
            "Property {}: Offset = {}, Length = {}",
            i, property_info.name_offset, property_info.length
        );

        // Validate the offset and length
        if property_info.name_offset as usize >= buffer.len() {
            return Err(format!("Property name offset out of bounds for property {}", i).into());
        }

        if property_info.length == 0 || property_info.length % 2 != 0 {
            return Err(format!(
                "Invalid property name length for property {}: {}",
                i, property_info.length
            )
            .into());
        }

        // Calculate the pointer to the property name
        let property_name_ptr =
            unsafe { buffer.as_ptr().add(property_info.name_offset as usize) as *const u16 };

        // Convert PropertyNameLength from bytes to the number of UTF-16 characters
        let property_name_length = property_info.length / std::mem::size_of::<u16>() as u32;

        // Convert the property name to a Rust String
        let property_name = unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(
                property_name_ptr,
                property_name_length as usize,
            ))
        };

        properties.push(property_name);
    }

    Ok(properties)
}
