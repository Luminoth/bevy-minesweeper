mod components;
mod events;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

use plugins::debug::*;
use plugins::states::*;
use states::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;

fn setup(mut _commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();
}

fn main() {
    let mut app = App::new();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper Tutorial!".to_owned(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        vsync: false,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin);

    // egui
    app.insert_resource(EguiSettings { scale_factor: 0.75 })
        .add_plugin(EguiPlugin);

    // inspector
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new());

    // plugins
    app.add_plugin(DebugPlugin).add_plugins(StatesPlugins);

    // initial game state
    app.add_state(GameState::Game);

    // main setup
    app.add_startup_system(setup);

    app.run();
}
