use std::fmt;

/// Basketball Protocol Parser
/// 
/// This parser handles the basketball scorepad network protocol
/// for parsing game data including scores, time, periods, and other game state information.

#[derive(Debug, Clone, PartialEq)]
pub struct BasketballProtocol {
    pub home_score: u16,
    pub away_score: u16,
    pub period: u8,
    pub time_minutes: u8,
    pub time_seconds: u8,
    pub home_fouls: u8,
    pub away_fouls: u8,
    pub home_timeouts: u8,
    pub away_timeouts: u8,
    pub possession: Possession,
    pub game_state: GameState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Possession {
    Home,
    Away,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    PreGame,
    Running,
    Paused,
    Halftime,
    Overtime,
    Final,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidLength(usize),
    InvalidProtocolId(u8),
    InvalidPeriod(u8),
    InvalidTime(u8, u8),
    InvalidPossession(u8),
    InvalidGameState(u8),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidLength(len) => write!(f, "Invalid data length: {}", len),
            ParseError::InvalidProtocolId(id) => write!(f, "Invalid protocol ID: 0x{:02X}", id),
            ParseError::InvalidPeriod(period) => write!(f, "Invalid period: {}", period),
            ParseError::InvalidTime(min, sec) => write!(f, "Invalid time: {}:{:02}", min, sec),
            ParseError::InvalidPossession(val) => write!(f, "Invalid possession value: {}", val),
            ParseError::InvalidGameState(val) => write!(f, "Invalid game state value: {}", val),
        }
    }
}

impl std::error::Error for ParseError {}

impl BasketballProtocol {
    /// Parse raw bytes into a BasketballProtocol structure
    /// 
    /// Expected format (minimum 13 bytes):
    /// - Byte 0: Protocol ID (0x01 for basketball)
    /// - Bytes 1-2: Home score (little-endian u16)
    /// - Bytes 3-4: Away score (little-endian u16)
    /// - Byte 5: Period (1-4 for regular, 5+ for overtime)
    /// - Byte 6: Time minutes (0-99)
    /// - Byte 7: Time seconds (0-59)
    /// - Byte 8: Home fouls (0-99)
    /// - Byte 9: Away fouls (0-99)
    /// - Byte 10: Home timeouts remaining (0-9)
    /// - Byte 11: Away timeouts remaining (0-9)
    /// - Byte 12: Possession (0=None, 1=Home, 2=Away)
    /// - Byte 13: Game state (0=PreGame, 1=Running, 2=Paused, 3=Halftime, 4=Overtime, 5=Final)
    pub fn parse(data: &[u8]) -> Result<Self, ParseError> {
        // Validate minimum length
        if data.len() < 14 {
            return Err(ParseError::InvalidLength(data.len()));
        }

        // Validate protocol ID
        if data[0] != 0x01 {
            return Err(ParseError::InvalidProtocolId(data[0]));
        }

        // Parse scores (little-endian)
        let home_score = u16::from_le_bytes([data[1], data[2]]);
        let away_score = u16::from_le_bytes([data[3], data[4]]);

        // Parse period
        let period = data[5];
        if period == 0 || period > 10 {
            return Err(ParseError::InvalidPeriod(period));
        }

        // Parse time
        let time_minutes = data[6];
        let time_seconds = data[7];
        if time_seconds >= 60 {
            return Err(ParseError::InvalidTime(time_minutes, time_seconds));
        }

        // Parse fouls
        let home_fouls = data[8];
        let away_fouls = data[9];

        // Parse timeouts
        let home_timeouts = data[10];
        let away_timeouts = data[11];

        // Parse possession
        let possession = match data[12] {
            0 => Possession::None,
            1 => Possession::Home,
            2 => Possession::Away,
            val => return Err(ParseError::InvalidPossession(val)),
        };

        // Parse game state
        let game_state = match data[13] {
            0 => GameState::PreGame,
            1 => GameState::Running,
            2 => GameState::Paused,
            3 => GameState::Halftime,
            4 => GameState::Overtime,
            5 => GameState::Final,
            val => return Err(ParseError::InvalidGameState(val)),
        };

        Ok(BasketballProtocol {
            home_score,
            away_score,
            period,
            time_minutes,
            time_seconds,
            home_fouls,
            away_fouls,
            home_timeouts,
            away_timeouts,
            possession,
            game_state,
        })
    }

    /// Serialize the protocol back to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(14);
        
        // Protocol ID
        bytes.push(0x01);
        
        // Scores (little-endian)
        bytes.extend_from_slice(&self.home_score.to_le_bytes());
        bytes.extend_from_slice(&self.away_score.to_le_bytes());
        
        // Period
        bytes.push(self.period);
        
        // Time
        bytes.push(self.time_minutes);
        bytes.push(self.time_seconds);
        
        // Fouls
        bytes.push(self.home_fouls);
        bytes.push(self.away_fouls);
        
        // Timeouts
        bytes.push(self.home_timeouts);
        bytes.push(self.away_timeouts);
        
        // Possession
        bytes.push(match self.possession {
            Possession::None => 0,
            Possession::Home => 1,
            Possession::Away => 2,
        });
        
        // Game state
        bytes.push(match self.game_state {
            GameState::PreGame => 0,
            GameState::Running => 1,
            GameState::Paused => 2,
            GameState::Halftime => 3,
            GameState::Overtime => 4,
            GameState::Final => 5,
        });
        
        bytes
    }

    /// Format time as MM:SS string
    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}", self.time_minutes, self.time_seconds)
    }

    /// Get period name
    pub fn period_name(&self) -> String {
        match self.period {
            1 => "1st Quarter".to_string(),
            2 => "2nd Quarter".to_string(),
            3 => "3rd Quarter".to_string(),
            4 => "4th Quarter".to_string(),
            n if n > 4 => format!("OT{}", n - 4),
            _ => "Unknown".to_string(),
        }
    }

    /// Check if game is in overtime
    pub fn is_overtime(&self) -> bool {
        self.period > 4
    }

    /// Check if game is finished
    pub fn is_finished(&self) -> bool {
        matches!(self.game_state, GameState::Final)
    }
}

impl Default for BasketballProtocol {
    fn default() -> Self {
        BasketballProtocol {
            home_score: 0,
            away_score: 0,
            period: 1,
            time_minutes: 12,
            time_seconds: 0,
            home_fouls: 0,
            away_fouls: 0,
            home_timeouts: 7,
            away_timeouts: 7,
            possession: Possession::None,
            game_state: GameState::PreGame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_protocol() {
        let data = vec![
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

        let protocol = BasketballProtocol::parse(&data).unwrap();
        assert_eq!(protocol.home_score, 80);
        assert_eq!(protocol.away_score, 74);
        assert_eq!(protocol.period, 4);
        assert_eq!(protocol.time_minutes, 2);
        assert_eq!(protocol.time_seconds, 30);
        assert_eq!(protocol.possession, Possession::Home);
        assert_eq!(protocol.game_state, GameState::Running);
    }

    #[test]
    fn test_parse_invalid_length() {
        let data = vec![0x01, 0x50, 0x00];
        assert!(matches!(
            BasketballProtocol::parse(&data),
            Err(ParseError::InvalidLength(_))
        ));
    }

    #[test]
    fn test_parse_invalid_protocol_id() {
        let data = vec![0x02, 0x50, 0x00, 0x4A, 0x00, 0x04, 0x02, 0x1E, 0x04, 0x05, 0x03, 0x02, 0x01, 0x01];
        assert!(matches!(
            BasketballProtocol::parse(&data),
            Err(ParseError::InvalidProtocolId(_))
        ));
    }

    #[test]
    fn test_serialize_deserialize() {
        let original = BasketballProtocol {
            home_score: 95,
            away_score: 88,
            period: 4,
            time_minutes: 0,
            time_seconds: 45,
            home_fouls: 3,
            away_fouls: 6,
            home_timeouts: 1,
            away_timeouts: 2,
            possession: Possession::Away,
            game_state: GameState::Running,
        };

        let bytes = original.to_bytes();
        let parsed = BasketballProtocol::parse(&bytes).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_format_time() {
        let protocol = BasketballProtocol {
            time_minutes: 5,
            time_seconds: 30,
            ..Default::default()
        };
        assert_eq!(protocol.format_time(), "05:30");
    }

    #[test]
    fn test_period_name() {
        let mut protocol = BasketballProtocol::default();
        
        protocol.period = 1;
        assert_eq!(protocol.period_name(), "1st Quarter");
        
        protocol.period = 4;
        assert_eq!(protocol.period_name(), "4th Quarter");
        
        protocol.period = 5;
        assert_eq!(protocol.period_name(), "OT1");
        
        protocol.period = 6;
        assert_eq!(protocol.period_name(), "OT2");
    }

    #[test]
    fn test_overtime_detection() {
        let mut protocol = BasketballProtocol::default();
        
        protocol.period = 4;
        assert!(!protocol.is_overtime());
        
        protocol.period = 5;
        assert!(protocol.is_overtime());
    }
}
