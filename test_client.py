#!/usr/bin/env python3
"""
Basketball Protocol TCP Client
Sends basketball game data to the TCP server for parsing
"""

import socket
import time
import struct

def send_basketball_data(host='127.0.0.1', port=8888):
    """Send basketball protocol data to the server"""
    
    # Create socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    try:
        # Connect to server
        print(f"Connecting to {host}:{port}...")
        sock.connect((host, port))
        print("Connected!\n")
        
        # Example game progression
        game_updates = [
            {
                'home_score': 0,
                'away_score': 0,
                'period': 1,
                'time_min': 12,
                'time_sec': 0,
                'home_fouls': 0,
                'away_fouls': 0,
                'home_timeouts': 7,
                'away_timeouts': 7,
                'possession': 0,  # None
                'game_state': 0,  # PreGame
            },
            {
                'home_score': 12,
                'away_score': 8,
                'period': 1,
                'time_min': 8,
                'time_sec': 30,
                'home_fouls': 1,
                'away_fouls': 2,
                'home_timeouts': 7,
                'away_timeouts': 7,
                'possession': 1,  # Home
                'game_state': 1,  # Running
            },
            {
                'home_score': 28,
                'away_score': 24,
                'period': 2,
                'time_min': 6,
                'time_sec': 15,
                'home_fouls': 3,
                'away_fouls': 2,
                'home_timeouts': 6,
                'away_timeouts': 7,
                'possession': 2,  # Away
                'game_state': 1,  # Running
            },
            {
                'home_score': 45,
                'away_score': 42,
                'period': 2,
                'time_min': 0,
                'time_sec': 0,
                'home_fouls': 5,
                'away_fouls': 4,
                'home_timeouts': 5,
                'away_timeouts': 6,
                'possession': 0,  # None
                'game_state': 3,  # Halftime
            },
            {
                'home_score': 80,
                'away_score': 74,
                'period': 4,
                'time_min': 2,
                'time_sec': 30,
                'home_fouls': 4,
                'away_fouls': 5,
                'home_timeouts': 3,
                'away_timeouts': 2,
                'possession': 1,  # Home
                'game_state': 1,  # Running
            },
            {
                'home_score': 95,
                'away_score': 95,
                'period': 4,
                'time_min': 0,
                'time_sec': 0,
                'home_fouls': 6,
                'away_fouls': 6,
                'home_timeouts': 1,
                'away_timeouts': 0,
                'possession': 0,  # None
                'game_state': 4,  # Overtime
            },
            {
                'home_score': 105,
                'away_score': 102,
                'period': 5,
                'time_min': 3,
                'time_sec': 45,
                'home_fouls': 7,
                'away_fouls': 6,
                'home_timeouts': 1,
                'away_timeouts': 0,
                'possession': 2,  # Away
                'game_state': 1,  # Running
            },
            {
                'home_score': 110,
                'away_score': 108,
                'period': 5,
                'time_min': 0,
                'time_sec': 0,
                'home_fouls': 8,
                'away_fouls': 7,
                'home_timeouts': 0,
                'away_timeouts': 0,
                'possession': 0,  # None
                'game_state': 5,  # Final
            },
        ]
        
        # Send each update
        for i, update in enumerate(game_updates, 1):
            print(f"Sending update #{i}...")
            
            # Build protocol packet (14 bytes)
            # Protocol ID (1 byte) + Home Score (2 bytes LE) + Away Score (2 bytes LE) +
            # Period (1) + Time Min (1) + Time Sec (1) + Home Fouls (1) + Away Fouls (1) +
            # Home Timeouts (1) + Away Timeouts (1) + Possession (1) + Game State (1)
            
            packet = struct.pack(
                '<BHHBBBBBBBBB',
                0x01,  # Protocol ID
                update['home_score'],
                update['away_score'],
                update['period'],
                update['time_min'],
                update['time_sec'],
                update['home_fouls'],
                update['away_fouls'],
                update['home_timeouts'],
                update['away_timeouts'],
                update['possession'],
                update['game_state']
            )
            
            # Send packet
            sock.sendall(packet)
            
            # Wait for response
            response = sock.recv(1024).decode('utf-8', errors='ignore')
            print(f"Server response: {response.strip()}")
            print()
            
            # Wait before next update
            if i < len(game_updates):
                time.sleep(2)
        
        print("All updates sent successfully!")
        
    except Exception as e:
        print(f"Error: {e}")
    finally:
        sock.close()
        print("Connection closed.")

if __name__ == '__main__':
    send_basketball_data()
