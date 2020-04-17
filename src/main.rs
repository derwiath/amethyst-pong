mod systems;

use amethyst::{
    config::Config,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod config;
mod pong;

use crate::config::PongConfig;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");
    let config_path = config_dir.join("config.ron");
    let pong_cfg = <PongConfig as Config>::load(config_path).expect("failed to load game config");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window
                // and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "move_ball_system", &[])
        .with(
            systems::BounceSystem,
            "bounce_system",
            &["paddle_system", "move_ball_system"],
        )
        .with(
            systems::WinnerSystem,
            "winner_system",
            &["move_ball_system"],
        );

    let mut game = Application::build(assets_dir, Pong::default())?
        .with_resource(pong_cfg.arena)
        .with_resource(pong_cfg.ball)
        .build(game_data)?;
    game.run();

    Ok(())
}
