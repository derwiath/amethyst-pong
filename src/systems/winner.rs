use std::ops::Deref;

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
};

use crate::audio::{play_sample, Sounds};
use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            mut balls,
            mut locals,
            mut ui_text,
            mut score_board,
            score_text,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side.
                score_board.right = (score_board.right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.right) {
                    text.text = score_board.right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                score_board.left = (score_board.left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.left) {
                    text.text = score_board.left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                println!("Scores: {:^3} - {:^3}", score_board.left, score_board.right);
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                play_sample(
                    &sounds.score_sfx,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );
            }
        }
    }
}
