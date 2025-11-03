# Basketball Protocol Parser (Rust)

A Rust implementation of a basketball scorepad network protocol parser with TCP server support and live HTML/CSS overlay.

## Quick Start

```bash
# 1. Start the server + web overlay
cargo run server

# 2. In another terminal, send test data
python3 test_client.py

# 3. Open browser to see live overlay
# http://localhost:8080
```

Or use the demo script:
```bash
./demo.sh
```

## Features

- ✅ Parse basketball protocol messages (14 bytes)
- ✅ TCP server for receiving real-time game data
- ✅ **Live HTML/CSS overlay with real-time updates**
- ✅ Web API for accessing game state
- ✅ Support for scores, time, fouls, timeouts, possession, and game state
- ✅ Overtime detection
- ✅ Multiple game states (PreGame, Running, Paused, Halftime, Overtime, Final)
- ✅ Serialization and deserialization
- ✅ Comprehensive error handling
- ✅ Responsive design for different screen sizes

## Protocol Specification

The basketball protocol uses a 14-byte binary format:

| Byte(s) | Field | Type | Description |
|---------|-------|------|-------------|
| 0 | Protocol ID | u8 | Always 0x01 for basketball |
| 1-2 | Home Score | u16 (LE) | Home team score (0-999) |
| 3-4 | Away Score | u16 (LE) | Away team score (0-999) |
| 5 | Period | u8 | Quarter/Period (1-4 regular, 5+ overtime) |
| 6 | Time Minutes | u8 | Minutes remaining (0-99) |
| 7 | Time Seconds | u8 | Seconds remaining (0-59) |
| 8 | Home Fouls | u8 | Home team fouls (0-99) |
| 9 | Away Fouls | u8 | Away team fouls (0-99) |
| 10 | Home Timeouts | u8 | Home timeouts remaining (0-9) |
| 11 | Away Timeouts | u8 | Away timeouts remaining (0-9) |
| 12 | Possession | u8 | 0=None, 1=Home, 2=Away |
| 13 | Game State | u8 | 0=PreGame, 1=Running, 2=Paused, 3=Halftime, 4=Overtime, 5=Final |

## Usage

### Run Examples

```bash
cargo run
```

This will run through several examples demonstrating parsing and serialization.

### Start TCP Server + Web Overlay

```bash
cargo run server
```

This will start:
- **TCP Server** on `127.0.0.1:8888` - Receives basketball protocol messages
- **Web Server** on `127.0.0.1:8080` - Serves the live scoreboard overlay

Open http://localhost:8080 in your browser to see the live overlay!

### View the Overlay

1. Start the server: `cargo run server`
2. Open your browser to: http://localhost:8080
3. Send game data to the TCP server (port 8888)
4. Watch the overlay update in real-time!

**OBS Studio Integration:**
- Add a Browser Source in OBS
- Set URL to: `http://localhost:8080`
- Set Width: 1920, Height: 1080
- Check "Shutdown source when not visible"
- The overlay has a transparent background and will appear over your stream!

### Test with Client

In another terminal, run the Python test client:

```bash
python3 test_client.py
```

This will send a series of game updates to the server, simulating a complete basketball game from start to finish, including overtime!

### Manual Testing with netcat

You can also send raw bytes using netcat:

```bash
# Example: Home 80 - Away 74, 4th Quarter, 2:30 remaining
echo -ne '\x01\x50\x00\x4A\x00\x04\x02\x1E\x04\x05\x03\x02\x01\x01' | nc 127.0.0.1 8888
```

## Library Usage

```rust
use basketball_parser::BasketballProtocol;

// Parse incoming data
let data = vec![0x01, 0x50, 0x00, 0x4A, 0x00, 0x04, 0x02, 0x1E, 
                0x04, 0x05, 0x03, 0x02, 0x01, 0x01];
let protocol = BasketballProtocol::parse(&data)?;

println!("Score: {} - {}", protocol.home_score, protocol.away_score);
println!("Time: {}", protocol.format_time());
println!("Period: {}", protocol.period_name());

// Create and serialize
let game = BasketballProtocol {
    home_score: 95,
    away_score: 88,
    period: 4,
    time_minutes: 0,
    time_seconds: 45,
    // ... other fields
    ..Default::default()
};

let bytes = game.to_bytes();
```

## Running Tests

```bash
cargo test
```

## Architecture

### Modules

- **basketball_parser**: Core protocol parsing and serialization
- **tcp_server**: TCP server for streaming protocol data
- **web_server**: HTTP server for overlay and API
- **main**: CLI interface and examples

### Key Endpoints

- `http://localhost:8080/` - Live scoreboard overlay (HTML)
- `http://localhost:8080/api/state` - Current game state (JSON API)

### Key Types

- `BasketballProtocol`: Main protocol structure
- `Possession`: Enum for ball possession (Home, Away, None)
- `GameState`: Enum for game states (PreGame, Running, Paused, Halftime, Overtime, Final)
- `ParseError`: Error types for parsing failures

## Error Handling

The parser provides detailed error messages for:
- Invalid data length
- Invalid protocol ID
- Invalid period numbers
- Invalid time values
- Invalid possession values
- Invalid game state values

## Example Output

### Web Overlay Features

The overlay displays:
- ✨ **Real-time score updates** with pulse animations
- ⏱️ **Game clock** with warning colors when time is low
- 🏀 **Possession indicator** with pulsing animation
- 📊 **Team stats** (fouls, timeouts)
- 🎮 **Game state** (Running, Paused, Halftime, etc.)
- 🔥 **Overtime badge** when game goes to OT
- 🏁 **Final badge** when game ends
- 📱 **Responsive design** for different screen sizes

### Console Output

```
=== Basketball Protocol TCP Server + Web Overlay ===

🏀 TCP Server: 127.0.0.1:8888
🌐 Web Overlay: http://127.0.0.1:8080

Send basketball protocol data to TCP port 8888
Open http://localhost:8080 in your browser to see the overlay
Press Ctrl+C to stop.

📡 New connection from: 127.0.0.1:54321
📥 Received 14 bytes from 127.0.0.1:54321

✅ Successfully parsed protocol:
  Score: Home 80 - 74 Away
  Period: 4th Quarter
  Time: 02:30
  Fouls: Home 4 - 5 Away
  Timeouts: Home 3 - 2 Away
  Possession: Home
  Game State: Running
```

### API Response Example

```json
{
  "home_score": 80,
  "away_score": 74,
  "period": 4,
  "period_name": "4th Quarter",
  "time": "02:30",
  "home_fouls": 4,
  "away_fouls": 5,
  "home_timeouts": 3,
  "away_timeouts": 2,
  "possession": "Home",
  "game_state": "Running",
  "is_overtime": false,
  "is_finished": false
}
```

## License

This project is open source and available under the MIT License.
