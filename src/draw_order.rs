use bevy::prelude::*;

pub struct DrawOrderPlugin;

impl Plugin for DrawOrderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(normalise_z_values);
    }
}

#[derive(Debug, Copy, Clone, Component)]
pub struct DrawLayer {
    pub layer: u16,
}

impl DrawLayer {
    pub fn new(layer: u16) -> Self {
        DrawLayer { layer }
    }
}

fn normalise_z_values(
    mut query: Query<(&GlobalTransform, &mut Transform, &DrawLayer), With<Sprite>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
) {
    let (camera, camera_transform) = camera.get_single().unwrap();
    let height = windows.primary().height();

    for (global, mut transform, layer) in &mut query {
        let screen_coords = camera
            .world_to_viewport(camera_transform, global.translation())
            .expect("Error calculating screen coordinates from world coordinates");

        let scaled_y = screen_coords.y / height;
        transform.translation.z = 1. + layer.layer as f32 - scaled_y;
    }
}
