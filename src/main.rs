#![allow(unused_variables)]

mod draw_order;

use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, WindowMode};
use draw_order::*;

const UP: KeyCode = KeyCode::W;
const DOWN: KeyCode = KeyCode::S;
const LEFT: KeyCode = KeyCode::A;
const RIGHT: KeyCode = KeyCode::D;

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

const PLAYER_DEFAULT_SPEED: f32 = 200.;

#[derive(Debug, Copy, Clone, Component)]
enum OreType {
    Coal,
    Copper,
    Iron,
    Lunium,
}

#[derive(Debug, Copy, Clone, Component)]
struct OreVein {
    ore_type: OreType,
    amount: u32,
}

#[derive(Debug, Copy, Clone, Component)]
struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: PLAYER_DEFAULT_SPEED,
        }
    }
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let coal_ore = server.load("sprites/coal_ore.png");
    let player = server.load("sprites/ralsei deltarune.png");
    commands.spawn_bundle(Camera2dBundle::default());

    // Player Sprite (for draw order testing)
    commands
        .spawn_bundle(SpriteBundle {
            texture: player.clone(),
            transform: Transform::from_xyz(-100., -200., 1.),
            ..default()
        })
        .insert(DrawLayer::new(2));

    // Player
    commands
        .spawn_bundle(SpriteBundle {
            texture: player,
            transform: Transform::from_xyz(10., 10., 1.),
            ..default()
        })
        .insert(Player::default())
        .insert(DrawLayer::new(2));

    // Test Ore
    commands
        .spawn_bundle(SpriteBundle {
            texture: coal_ore,
            transform: Transform::from_xyz(10., 10., 1.),
            ..default()
        })
        .insert(OreVein {
            ore_type: OreType::Coal,
            amount: 150,
        })
        .insert(DrawLayer::new(1));
}

fn move_player(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    let delta = time.delta_seconds();
    let (mut transform, player) = query.get_single_mut().unwrap();

    let vspeed = (input.pressed(UP) as i32 - input.pressed(DOWN) as i32) as f32;
    let hspeed = (input.pressed(RIGHT) as i32 - input.pressed(LEFT) as i32) as f32;

    let scaling = if hspeed == 0. && vspeed == 0. {
        0.
    } else {
        1. / ((vspeed * vspeed + hspeed * hspeed).sqrt())
    };

    transform.translation.x += hspeed * scaling * player.speed * delta;
    transform.translation.y += vspeed * scaling * player.speed * delta;

    transform.translation.x = transform.translation.x.round();
    transform.translation.y = transform.translation.y.round();
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "overwatch 3".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(move_player)
        .add_plugin(DrawOrderPlugin)
        .run();
}
