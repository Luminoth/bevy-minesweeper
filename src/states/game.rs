use bevy::prelude::*;

use crate::components::*;
use crate::resources::tilemap::*;
use crate::resources::*;

pub fn setup(mut commands: Commands, mut random: ResMut<Random>) {
    // cameras
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical;

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    let mut tile_map = TileMap::new(20, 20);
    tile_map.set_bombs(40, &mut random);
    info!("{}", tile_map.get_debug_string());

    commands.insert_resource(tile_map);
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<TileMap>();
    commands.remove_resource::<ClearColor>();
}
