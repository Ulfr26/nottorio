#![allow(unused_variables)]
use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, WindowMode};

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

#[derive(Debug, Copy, Clone, Component)]
struct DebugText;

#[derive(Debug, Copy, Clone, Component)]
struct DrawLayer(u16);

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

    let text = commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "COORDINATES",
                TextStyle {
                    font: server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 30.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(0., 50., 10.),
            ..default()
        })
        .insert(DebugText)
        .id();

    // Test Player Sprite
    commands
        .spawn_bundle(SpriteBundle {
            texture: player.clone(),
            transform: Transform::from_xyz(-100., -200., 1.),
            ..default()
        })
        .insert(DrawLayer(2));

    // Player
    commands
        .spawn_bundle(SpriteBundle {
            texture: player,
            transform: Transform::from_xyz(10., 10., 1.),
            ..default()
        })
        .insert(Player::default())
        .insert(DrawLayer(2))
        .add_child(text);

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
        .insert(DrawLayer(1));
}

fn normalise_z_values(
    mut query: Query<(&GlobalTransform, &mut Transform, &DrawLayer), With<Sprite>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.get_single().unwrap();

    for (global, mut transform, layer) in &mut query {
        let screen_coords = camera
            .world_to_viewport(camera_transform, global.translation())
            .expect("Error calculating screen coordinates from world coordinates");

        let scaled_y = screen_coords.y / HEIGHT;
        transform.translation.z = 1. + layer.0 as f32 - scaled_y;
    }
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

fn update_coords_text(
    mut query: Query<(&Parent, &mut Text), With<DebugText>>,
    q_parent: Query<&GlobalTransform>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (parent, mut text) = query.get_single_mut().unwrap();
    let parent_trans = q_parent.get(parent.get()).unwrap();
    let (camera, camera_transform) = camera.get_single().unwrap();

    let screen_coords = camera.world_to_viewport(camera_transform, parent_trans.translation());

    text.sections[0].value = match screen_coords {
        Some(Vec2 { x, y }) => {
            let x = x.round();
            let y = y.round();
            format!("({x}, {y})")
        }

        None => "Error!".to_string(),
    };
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
        .add_system(update_coords_text)
        .add_system(normalise_z_values)
        .run();
}
