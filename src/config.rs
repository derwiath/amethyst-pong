use amethyst::core::math::Vector2;
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BallConfig {
    pub velocity: Vector2<f32>,
    pub radius: f32,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            velocity: Vector2::new(75.0, 50.0),
            radius: 2.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaddleConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            width: 4.0,
            height: 16.0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PongConfig {
    pub arena: ArenaConfig,
    pub ball: BallConfig,
    pub paddle: PaddleConfig,
}
