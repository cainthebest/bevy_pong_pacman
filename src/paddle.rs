use bevy::prelude::*;

use crate::{components::*, WinSize};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_paddles);
    }
}

fn spawn_paddles(mut commands: Commands, winsize: Res<WinSize>) {
    let width: f32 = 30.;
    let height: f32 = 150.;

    let paddle_positions_and_sides = [(-winsize.w / 2., true), (winsize.w / 2., false)];

    for &(position, is_left) in paddle_positions_and_sides.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(position, 0., 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::Rgba {
                        red: 1.,
                        green: 1.,
                        blue: 1.,
                        alpha: 1.,
                    },
                    custom_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player {})
            .insert(Velocity {
                x: 0.,

                y: 1.,
                bound: true,
            })
            .insert(Hitbox {
                area: Rect {
                    left: -width / 2.,
                    right: width / 2.,
                    top: height / 2.,
                    bottom: -height / 2.,
                },
            })
            .insert(Side { left: is_left });
    }
    debug!("spawned paddles");
}
