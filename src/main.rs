use std::net;
use std::process::exit;
use std::time;

use clap::Parser;

/// Checks if a Don't Starve Together game server is alive and reports it's approximate latency
#[derive(Parser, Debug)]
struct Connection {
    /// IP address of the DST server
    address: net::IpAddr,
    /// Game port of the DST server
    port: u16,
}

// Packet doc:
// https://wiki.bedrock.dev/servers/raknet-and-mcpe.html
const PING_PACKAGE: [u8; 33] = [
    0x01, // Unconnected Ping Packet
    0x00, 0x00, 0x00, 0x00, 0x77, 0x6f, 0x6c, 0x6c, // Client alive time [ms], is echoed
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78, // magic
    0x00, 0x00, 0x00, 0x77, 0x6f, 0x6c, 0x6b, 0x65, // Client ID
];

const PONG_START: [u8; 9] = [
    0x1C, // Unconnected Pong Packet
    0x00, 0x00, 0x00, 0x00, 0x77, 0x6f, 0x6c, 0x6c, // Echoed client alive time [ms]
];

fn main() {
    let args = Connection::parse();

    let socket = net::UdpSocket::bind("[::]:0").expect("Cannot bind to local interface");
    socket
        .connect(net::SocketAddr::new(args.address, args.port))
        .expect("DST server should be reachable using the given parameters");
    socket
        .set_read_timeout(Some(time::Duration::new(5, 0)))
        .expect("Unable to set network timeout");

    let ping_sent = time::Instant::now();
    socket
        .send(&PING_PACKAGE)
        .expect("DST server should be reachable using the given parameters");

    // Packet truncated to relevant parts
    let mut pong = [0; 17];

    match socket.recv(&mut pong) {
        Ok(_) => {
            if pong.starts_with(&PONG_START) {
                // Received Unconnected Pong Packet
                let duration = ping_sent.elapsed();

                let mut server_guid = format!("{:X}", pong[9]);
                for x in pong[10..].iter() {
                    server_guid = format!("{} {:X}", server_guid, x);
                }

                println!("{:12} {}:{}", "Host:", args.address, args.port);
                println!("Server GUID: {}", &server_guid);
                println!("{:12} {} ms", "Time", duration.as_micros() as f32 / 1000.0);
            } else {
                eprintln!("Received malformed packet.");
                exit(1)
            }
        }
        Err(e) => {
            println!("{:8} {}:{}", "Host:", args.address, args.port);
            eprintln!("Timeout: {:?}", e.to_string());
            exit(1);
        }
    }
}
