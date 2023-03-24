use bevy::prelude::*;
use std::ops::Mul;

use bevy::math::*;

pub struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct PongBall;

#[derive(Component)]
pub struct Side {
    pub left: bool,
}
#[derive(Component)]
pub struct Hitbox {
    pub area: Rect<f32>,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub bound: bool,
}

impl Velocity {
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

pub struct BallPlugin;

fn spawn_balls(mut commands: Commands) {
    let radius: f32 = 15.;
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::Rgba {
                    red: (1.),
                    green: (1.),
                    blue: (1.),
                    alpha: (1.),
                },
                custom_size: Some(Vec2::new(radius, radius)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            x: 5.,
            y: 0.,
            bound: false,
        })
        .insert(PongBall {})
        .insert(Hitbox {
            area: Rect {
                left: -radius / 2.,
                right: radius / 2.,
                top: radius / 2.,
                bottom: -radius / 2.,
            },
        });
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_balls);
    }
}

pub struct PaddlePlugin;

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
}

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_paddles);
    }
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size: WinSize = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}

fn run_movement_tick(translation: &mut Vec3, velocity: &Velocity) {
    translation.y += velocity.y;
    translation.x += velocity.x;
}

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
fn intersects(pos1: Vec3, rect1: Rect<f32>, pos2: Vec3, rect2: Rect<f32>) -> bool {
    let x_intersection =
        rect1.left + pos1.x <= rect2.right + pos2.x && rect1.right + pos1.x >= rect2.left + pos2.x;
    let y_intersection =
        rect1.bottom + pos1.y <= rect2.top + pos2.y && rect1.top + pos1.y >= rect2.bottom + pos2.y;

    x_intersection && y_intersection
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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Rgba {
            red: (0.),
            green: (0.),
            blue: (0.),
            alpha: (1.0),
        }))
        .insert_resource(WindowDescriptor {
            title: "Pong Pacman!".to_string(),
            width: 700.0,
            height: 700.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(setup)
        .add_system(move_all_velocity_objects)
        .add_system(move_all_players.after(move_all_velocity_objects))
        .add_system(collision_ball.after(move_all_velocity_objects))
        .run();
}
