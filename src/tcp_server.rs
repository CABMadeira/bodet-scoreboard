use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::basketball_parser::{BasketballProtocol, ParseError};

/// TCP Server for Basketball Protocol
pub struct BasketballServer {
    address: String,
    current_state: Arc<Mutex<Option<BasketballProtocol>>>,
}

impl BasketballServer {
    /// Create a new server instance
    pub fn new(address: &str) -> Self {
        BasketballServer {
            address: address.to_string(),
            current_state: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new server instance with shared state
    pub fn with_shared_state(address: &str, state: Arc<Mutex<Option<BasketballProtocol>>>) -> Self {
        BasketballServer {
            address: address.to_string(),
            current_state: state,
        }
    }

    /// Start the server and listen for connections
    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("🏀 Basketball Protocol Server listening on {}", self.address);
        println!("Waiting for connections...\n");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let state = Arc::clone(&self.current_state);
                    thread::spawn(move || {
                        if let Err(e) = handle_client(stream, state) {
                            eprintln!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Get the current game state
    pub fn get_current_state(&self) -> Option<BasketballProtocol> {
        self.current_state.lock().unwrap().clone()
    }
}

/// Handle a single client connection
fn handle_client(
    mut stream: TcpStream,
    state: Arc<Mutex<Option<BasketballProtocol>>>,
) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("📡 New connection from: {}", peer_addr);

    // Set read timeout to prevent hanging
    stream.set_read_timeout(Some(Duration::from_secs(300)))?;

    let mut buffer = [0u8; 1024];
    let mut accumulated_data = Vec::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed
                println!("❌ Connection closed by: {}", peer_addr);
                break;
            }
            Ok(n) => {
                // Accumulate received data
                accumulated_data.extend_from_slice(&buffer[..n]);
                println!("📥 Received {} bytes from {}", n, peer_addr);

                // Try to parse complete protocol messages (14 bytes each)
                while accumulated_data.len() >= 14 {
                    let packet = accumulated_data[..14].to_vec();
                    accumulated_data.drain(..14);

                    match BasketballProtocol::parse(&packet) {
                        Ok(protocol) => {
                            println!("\n✅ Successfully parsed protocol:");
                            display_protocol(&protocol);

                            // Update shared state
                            *state.lock().unwrap() = Some(protocol.clone());

                            // Send acknowledgment back to client
                            let ack = b"ACK\n";
                            if let Err(e) = stream.write_all(ack) {
                                eprintln!("Failed to send ACK: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Parse error: {}", e);
                            eprintln!("   Raw bytes: {:02X?}", packet);

                            // Send error response
                            let err_msg = format!("ERROR: {}\n", e);
                            if let Err(e) = stream.write_all(err_msg.as_bytes()) {
                                eprintln!("Failed to send error message: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }

    Ok(())
}

/// Display protocol information
fn display_protocol(protocol: &BasketballProtocol) {
    println!("  Score: Home {} - {} Away", protocol.home_score, protocol.away_score);
    println!("  Period: {}", protocol.period_name());
    println!("  Time: {}", protocol.format_time());
    println!("  Fouls: Home {} - {} Away", protocol.home_fouls, protocol.away_fouls);
    println!("  Timeouts: Home {} - {} Away", protocol.home_timeouts, protocol.away_timeouts);
    println!("  Possession: {:?}", protocol.possession);
    println!("  Game State: {:?}", protocol.game_state);

    if protocol.is_overtime() {
        println!("  ⚠️  Game is in OVERTIME!");
    }

    if protocol.is_finished() {
        println!("  🏁 Game is FINISHED!");
    }
    println!();
}

/// Parse streaming data that may contain multiple protocol messages
pub fn parse_stream(data: &[u8]) -> Result<Vec<BasketballProtocol>, ParseError> {
    let mut protocols = Vec::new();
    let mut offset = 0;

    while offset + 14 <= data.len() {
        let packet = &data[offset..offset + 14];
        match BasketballProtocol::parse(packet) {
            Ok(protocol) => protocols.push(protocol),
            Err(e) => return Err(e),
        }
        offset += 14;
    }

    Ok(protocols)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basketball_parser::Possession;

    #[test]
    fn test_parse_stream_single_message() {
        let data = vec![
            0x01, 0x50, 0x00, 0x4A, 0x00, 0x04, 0x02, 0x1E,
            0x04, 0x05, 0x03, 0x02, 0x01, 0x01,
        ];

        let protocols = parse_stream(&data).unwrap();
        assert_eq!(protocols.len(), 1);
        assert_eq!(protocols[0].home_score, 80);
        assert_eq!(protocols[0].away_score, 74);
    }

    #[test]
    fn test_parse_stream_multiple_messages() {
        let mut data = Vec::new();
        
        // First message
        data.extend_from_slice(&[
            0x01, 0x50, 0x00, 0x4A, 0x00, 0x04, 0x02, 0x1E,
            0x04, 0x05, 0x03, 0x02, 0x01, 0x01,
        ]);
        
        // Second message
        data.extend_from_slice(&[
            0x01, 0x52, 0x00, 0x4A, 0x00, 0x04, 0x02, 0x00,
            0x04, 0x05, 0x03, 0x02, 0x02, 0x01,
        ]);

        let protocols = parse_stream(&data).unwrap();
        assert_eq!(protocols.len(), 2);
        assert_eq!(protocols[0].home_score, 80);
        assert_eq!(protocols[1].home_score, 82);
        assert_eq!(protocols[1].possession, Possession::Away);
    }

    #[test]
    fn test_parse_stream_incomplete_message() {
        let data = vec![
            0x01, 0x50, 0x00, 0x4A, 0x00, 0x04, 0x02,
        ];

        let protocols = parse_stream(&data).unwrap();
        assert_eq!(protocols.len(), 0); // Incomplete message ignored
    }
}
