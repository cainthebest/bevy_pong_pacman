use std::ops::Mul;

use bevy::math::*;
use bevy::prelude::*;

use crate::{
    components::{Hitbox, Player, PongBall, Side, Velocity},
    WinSize,
};

pub fn move_all_velocity_objects(
    winsize: Res<WinSize>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        run_movement_tick(&mut transform.translation, velocity);

        transform.translation.x = transform
            .translation
            .x
            .clamp(-winsize.w / 2., winsize.w / 2.);

        transform.translation.y = transform
            .translation
            .y
            .clamp(-winsize.h / 2., winsize.h / 2.);
    }
}

pub fn move_all_players(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Side, &mut Velocity, With<Player>)>,
) {
    for (side, mut velocity, _) in query.iter_mut() {
        let (up_key, down_key) = if side.left {
            (KeyCode::W, KeyCode::S)
        } else {
            (KeyCode::O, KeyCode::L)
        };

        velocity.y = 5. * (keys.pressed(up_key) as i32 - keys.pressed(down_key) as i32) as f32;
    }
}

pub fn collision_ball(
    mut ball_query: Query<(&Transform, &mut Velocity, &Hitbox, Without<Player>)>,
    player_query: Query<(&Transform, &Hitbox, Without<PongBall>)>,
    winsize: Res<WinSize>,
) {
    let (transform, mut velocity, ball_hitbox, _) = ball_query.single_mut();

    let mut translation: Vec3 = transform.translation;

    for (tf, player_hitbox, _) in player_query.iter() {
        if intersects(
            translation,
            ball_hitbox.area,
            tf.translation,
            player_hitbox.area,
        ) {
            let new_vel: Vec2 = Vec2::new(
                translation.x - tf.translation.x,
                translation.y - tf.translation.y,
            );
            let normalized_vel = new_vel.normalize().mul(3.85);

            velocity.set(normalized_vel.x, normalized_vel.y);
            let mut i = 0;
            while i < 3 {
                translation.y += velocity.y;
                translation.x += velocity.x;

                if !intersects(
                    translation,
                    ball_hitbox.area,
                    tf.translation,
                    player_hitbox.area,
                ) {
                    break;
                }
                i += 1;
            }
        }
    }

    if translation.y.abs() == winsize.h / 2. {
        velocity.y = -velocity.y;
    }
    if translation.x.abs() == winsize.w / 2. {
        let new_vel = Vec2::new(-translation.x, -translation.y)
            .normalize()
            .mul(3.85);
        velocity.set(new_vel.x, new_vel.y);
    }
}

fn intersects(pos1: Vec3, rect1: Rect<f32>, pos2: Vec3, rect2: Rect<f32>) -> bool {
    let x_intersection =
        rect1.left + pos1.x <= rect2.right + pos2.x && rect1.right + pos1.x >= rect2.left + pos2.x;
    let y_intersection =
        rect1.bottom + pos1.y <= rect2.top + pos2.y && rect1.top + pos1.y >= rect2.bottom + pos2.y;

    x_intersection && y_intersection
}

fn run_movement_tick(translation: &mut Vec3, velocity: &Velocity) {
    translation.y += velocity.y;
    translation.x += velocity.x;
}
