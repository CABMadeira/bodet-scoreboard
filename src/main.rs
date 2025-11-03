mod basketball_parser;
mod tcp_server;
mod web_server;

use basketball_parser::{BasketballProtocol, GameState, Possession};
use tcp_server::BasketballServer;
use web_server::WebServer;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, warn, error};
use env_logger::Env;

fn main() {
    // Initialize logger (reads RUST_LOG if set, defaults to `info` level)
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    
    // Check if server mode is requested
    if args.len() > 1 && args[1] == "server" {
        run_server();
        return;
    }
    
}

fn run_server() {
    info!("=== Basketball Protocol TCP Server + Web Overlay ===\n");
    
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
            error!("Web server error: {}", e);
        }
    });
    

    info!("TCP Server: {}", tcp_address);
    info!("Web Overlay: http://{}", web_address);
    info!("Send basketball protocol data to TCP port 8888");
    info!("Open http://localhost:8080 in your browser to see the overlay");
    info!("Press Ctrl+C to stop.");
    
    // Start TCP server with shared state
    let tcp_server = BasketballServer::with_shared_state(tcp_address, state);
    if let Err(e) = tcp_server.start() {
        error!("TCP server error: {}", e);
        std::process::exit(1);
    }
}


fn display_game_info(protocol: &BasketballProtocol) {
    info!("Score: Home {} - {} Away", protocol.home_score, protocol.away_score);
    info!("Period: {}", protocol.period_name());
    info!("Time: {}", protocol.format_time());
    info!("Fouls: Home {} - {} Away", protocol.home_fouls, protocol.away_fouls);
    info!("Timeouts: Home {} - {} Away", protocol.home_timeouts, protocol.away_timeouts);
    info!("Possession: {:?}", protocol.possession);
    info!("Game State: {:?}", protocol.game_state);
    
    if protocol.is_overtime() {
        warn!("Game is in OVERTIME!");
    }
    
    if protocol.is_finished() {
        info!("Game is FINISHED!");
        let winner = if protocol.home_score > protocol.away_score {
            "Home"
        } else if protocol.away_score > protocol.home_score {
            "Away"
        } else {
            "Tie"
        };
        info!("Winner: {}", winner);
    }
}
