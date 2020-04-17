use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::config::ArenaConfig;
use crate::pong::{Paddle, Side, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut transforms, paddles, input, arena_cfg): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let y = (transform.translation().y + scaled_amount)
                    .min(arena_cfg.height - PADDLE_HEIGHT * 0.5)
                    .max(PADDLE_HEIGHT * 0.5);
                transform.set_translation_y(y);
            }
        }
    }
}
