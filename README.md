# EDR Windows client app
Basic EDR which collects system events such as process_creation, file_written,  network_connection, dns_request, from multiple ETW providers, using QUIC to send data.

## Installation

1. Build the project:

    ```
    cargo build
    ```

## Usage

1. Run the project:

    ```
    cargo run -- -f <PROVIDERS_FILE> -i <"1.1.1.1"> -p <3333> -t <5> 
    ```

## Note

1. The version is not working properly.
2. Added new function `as_raw_ptr_hack` to `ferrisetw-1.2.0\src\native\etw_types\event_record.rs`.
3. Used `TdhGetEventInformation` struct to collect fields of the event.
4. All events stored in hash table `Hash <key= event_id, value = vector of fields>`.
5. Added QUIC client to send serialized hashtable.

