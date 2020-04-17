use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            width: 100.0,
            height: 100.0,
        }
    }
}
