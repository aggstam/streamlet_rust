use native_tls::TlsConnector;
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::{Instant, SystemTime},
};

use serde_json::Value;

// Clock sync parameters
const RETRIES: u8 = 10;
const WORLDTIMEAPI_ADDRESS: &str = "worldtimeapi.org";
const WORLDTIMEAPI_ADDRESS_WITH_PORT: &str = "worldtimeapi.org:443";
const WORLDTIMEAPI_PAYLOAD: &[u8; 88] = b"GET /api/timezone/Etc/UTC HTTP/1.1\r\nHost: worldtimeapi.org\r\nAccept: application/json\r\n\r\n";
const NTP_ADDRESS: &str = "0.pool.ntp.org:123";
const EPOCH: u64 = 2208988800; //1900

// Raw https request execution for worldtimeapi
fn worldtimeapi_request() -> Value {
    // Create connection
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(WORLDTIMEAPI_ADDRESS_WITH_PORT).unwrap();
    let mut stream = connector.connect(WORLDTIMEAPI_ADDRESS, stream).unwrap();
    stream.write_all(WORLDTIMEAPI_PAYLOAD).unwrap();

    // Execute request
    let mut res = vec![0_u8; 1024];
    stream.read(&mut res).unwrap();

    // Parse response
    let reply = String::from_utf8(res).unwrap();
    let lines = reply.split('\n');
    // JSON data exist in last row of response
    let last = lines.last().unwrap().trim_matches(char::from(0));
    println!("worldtimeapi json response: {:#?}", last);
    let reply = serde_json::from_str(last).unwrap();

    reply
}

// This is a very simple check to verify that system time is correct.
// Retry loop is used to in case discrepancies are found.
// If all retries fail, system clock is considered invalid.
pub fn check_clock() {
    println!("System clock check started...");
    let mut r = 0;
    while r < RETRIES {
        if !clock_check() {
            println!("Error during clock check, retrying...");
            r += 1;
            continue
        };
        break
    }

    println!("System clock check finished. Retries: {:#?}", r);
    match r {
        RETRIES => panic!("Invalid system clock."),
        _ => (),
    }
}

fn clock_check() -> bool {
    // Start elapsed time counter to cover for all requests and processing time
    let requests_start = Instant::now();
    // Poll worldtimeapi.org for current UTC timestamp
    let worldtimeapi_response = worldtimeapi_request();

    // Start elapsed time counter to cover for ntp request and processing time
    let ntp_request_start = Instant::now();
    // Poll ntp.org for current timestamp
    let ntp_response: ntp::packet::Packet = ntp::request(NTP_ADDRESS).unwrap();

    // Extract worldtimeapi timestamp from json
    let mut worldtimeapi_time = worldtimeapi_response["unixtime"].as_u64().unwrap();

    // Remove 1900 epoch to reach UTC timestamp for ntp timestamp
    let mut ntp_time = ntp_response.transmit_time.sec as u64 - EPOCH;

    // Add elapsed time to respone times
    ntp_time += ntp_request_start.elapsed().as_secs();
    worldtimeapi_time += requests_start.elapsed().as_secs();

    // Current system time
    let system_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    println!("worldtimeapi_time: {:#?}", worldtimeapi_time);
    println!("ntp_time: {:#?}", ntp_time);
    println!("system_time: {:#?}", system_time);

    // We verify that system time is equal to worldtimeapi and ntp
    (system_time == worldtimeapi_time) && (system_time == ntp_time)
}
