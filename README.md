## Scoreboard (Basketball Protocol Parser)

Scoreboard is a Rust implementation of a basketball scorepad protocol parser with a TCP server and a live HTML/CSS overlay. It receives 14-byte protocol messages, maintains current game state, exposes a small JSON API, and provides a browser overlay suitable for streaming or local display.

Table of contents
-----------------

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Quick start](#quick-start)
- [Protocol specification](#protocol-specification)
- [Running and testing](#running-and-testing)
- [API](#api)
- [Development notes](#development-notes)
- [Contributing](#contributing)
- [License](#license)

Features
--------

- Parse and validate a 14-byte basketball protocol message
- TCP server to receive live game updates
- Web server that serves a real-time overlay (HTML/JS/CSS)
- JSON API for current game state
- Support for scores, clock, fouls, timeouts, possession, and game state
- Overtime detection and final state handling

Prerequisites
-------------

- Rust toolchain (stable) and Cargo
- Python 3 (optional) for the provided test client
- A modern web browser to view the overlay

Quick start
-----------

Build the project:

```bash
cargo build --release
```

Start the TCP + web servers (default addresses):

```bash
cargo run -- server
```

Behavior:

- TCP server: 127.0.0.1:8888 (receives 14-byte messages)
- Web server: 127.0.0.1:8080 (serves overlay and JSON API)

Open http://localhost:8080 in a browser to view the live overlay.

Protocol specification
----------------------

Messages are fixed length (14 bytes). Fields use little-endian where applicable.

| Byte(s) | Field | Type | Notes |
|--------:|:------|:-----:|:------|
| 0 | protocol_id | u8 | Should be 0x01 for basketball messages |
| 1-2 | home_score | u16 (LE) | 0–999 typical |
| 3-4 | away_score | u16 (LE) | 0–999 typical |
| 5 | period | u8 | 1–4 regular, 5+ overtime |
| 6 | minutes | u8 | 0–99 |
| 7 | seconds | u8 | 0–59 |
| 8 | home_fouls | u8 | 0–99 |
| 9 | away_fouls | u8 | 0–99 |
| 10 | home_timeouts | u8 | 0–9 |
| 11 | away_timeouts | u8 | 0–9 |
| 12 | possession | u8 | 0=None, 1=Home, 2=Away |
| 13 | game_state | u8 | 0=PreGame, 1=Running, 2=Paused, 3=Halftime, 4=Overtime, 5=Final |

Running and testing
-------------------

Send test data using the provided Python client (if present):

```bash
python3 test_client.py
```

Manual example (netcat):

```bash
# Example bytes for Home 80 - Away 74, 4th period, 2:30 remaining
echo -ne '\x01\x50\x00\x4A\x00\x04\x02\x1E\x04\x05\x03\x02\x01\x01' | nc 127.0.0.1 8888
```

API
---

The web server exposes a small JSON API for the current game state:

- GET /api/state — returns the current game state as JSON (scores, clock, fouls, timeouts, possession, game state flags).

Development notes
-----------------

Project layout (high level):

- `src/` — main application and modules
  - `basketball_parser` — parsing and serialization logic
  - `tcp_server` — TCP listener and connection handling
  - `web_server` — static overlay and JSON API
- `static/` — `overlay.html`, `overlay.css`, `overlay.js`
- `send_hex_stream_tcp.py`, `test_client.py` — helper/test scripts

Error handling
--------------

The parser returns explicit errors for invalid input: wrong message length, invalid protocol ID, invalid numeric ranges for time or period, and invalid enum values for possession or game state.

Contributing
------------

Contributions are welcome. For small changes, open a pull request with a clear description. For larger changes, open an issue first to discuss the design.

License
-------

This project is licensed under the MIT License. See the `LICENSE` file for details.

Contact
-------

For questions or issues, please open an issue in this repository.


