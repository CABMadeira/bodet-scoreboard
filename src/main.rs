mod basketball_parser;
mod tcp_server;
mod web_server;

use basketball_parser::{BasketballProtocol, GameState, Possession};
use tcp_server::BasketballServer;
use web_server::WebServer;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check if server mode is requested
    if args.len() > 1 && args[1] == "server" {
        run_server();
        return;
    }
    
    // Run examples
    run_examples();
}

fn run_server() {
    println!("=== Basketball Protocol TCP Server + Web Overlay ===\n");
    
    // Shared state between TCP server and web server
    let state = Arc::new(Mutex::new(None));
    
    let tcp_address = "127.0.0.1:8888";
    let web_address = "127.0.0.1:8080";
    
    // Start web server in a separate thread
    let web_state = Arc::clone(&state);
    let web_addr = web_address.to_string();
    thread::spawn(move || {
        let web_server = WebServer::new(&web_addr, web_state);
        if let Err(e) = web_server.start() {
            eprintln!("Web server error: {}", e);
        }
    });
    
    // Give web server time to start
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!("🏀 TCP Server: {}", tcp_address);
    println!("🌐 Web Overlay: http://{}", web_address);
    println!("\nSend basketball protocol data to TCP port 8888");
    println!("Open http://localhost:8080 in your browser to see the overlay");
    println!("Press Ctrl+C to stop.\n");
    
    // Start TCP server with shared state
    let tcp_server = BasketballServer::with_shared_state(tcp_address, state);
    if let Err(e) = tcp_server.start() {
        eprintln!("TCP server error: {}", e);
        std::process::exit(1);
    }
}

fn run_examples() {
    println!("=== Basketball Protocol Parser ===\n");
    println!("Run with 'cargo run server' to start TCP server\n");
    
    // Example 1: Parse a game in progress
    println!("Example 1: Game in progress");
    let game_data = vec![
        0x01,       // Protocol ID
        0x50, 0x00, // Home score: 80
        0x4A, 0x00, // Away score: 74
        0x04,       // Period: 4 (4th quarter)
        0x02,       // Time: 2 minutes
        0x1E,       // Time: 30 seconds
        0x04,       // Home fouls: 4
        0x05,       // Away fouls: 5
        0x03,       // Home timeouts: 3
        0x02,       // Away timeouts: 2
        0x01,       // Possession: Home
        0x01,       // Game state: Running
    ];
    
    match BasketballProtocol::parse(&game_data) {
        Ok(protocol) => {
            display_game_info(&protocol);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
    
    println!("\n---\n");
    
    // Example 2: Create and serialize a protocol
    println!("Example 2: Create and serialize");
    let new_game = BasketballProtocol {
        home_score: 105,
        away_score: 102,
        period: 5, // Overtime!
        time_minutes: 3,
        time_seconds: 45,
        home_fouls: 7,
        away_fouls: 6,
        home_timeouts: 1,
        away_timeouts: 0,
        possession: Possession::Away,
        game_state: GameState::Running,
    };
    
    display_game_info(&new_game);
    
    // Serialize and re-parse
    let serialized = new_game.to_bytes();
    println!("\nSerialized bytes: {:02X?}", serialized);
    
    match BasketballProtocol::parse(&serialized) {
        Ok(parsed) => {
            println!("Successfully re-parsed!");
            assert_eq!(new_game, parsed);
        }
        Err(e) => {
            eprintln!("Re-parse error: {}", e);
        }
    }
    
    println!("\n---\n");
    
    // Example 3: Default game (pre-game state)
    println!("Example 3: Default pre-game state");
    let default_game = BasketballProtocol::default();
    display_game_info(&default_game);
}

fn display_game_info(protocol: &BasketballProtocol) {
    println!("Score: Home {} - {} Away", protocol.home_score, protocol.away_score);
    println!("Period: {}", protocol.period_name());
    println!("Time: {}", protocol.format_time());
    println!("Fouls: Home {} - {} Away", protocol.home_fouls, protocol.away_fouls);
    println!("Timeouts: Home {} - {} Away", protocol.home_timeouts, protocol.away_timeouts);
    println!("Possession: {:?}", protocol.possession);
    println!("Game State: {:?}", protocol.game_state);
    
    if protocol.is_overtime() {
        println!("⚠️  Game is in OVERTIME!");
    }
    
    if protocol.is_finished() {
        println!("🏁 Game is FINISHED!");
        let winner = if protocol.home_score > protocol.away_score {
            "Home"
        } else if protocol.away_score > protocol.home_score {
            "Away"
        } else {
            "Tie"
        };
        println!("Winner: {}", winner);
    }
}
