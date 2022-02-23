use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::states;
use crate::states::*;

pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GameStatePlugin);
    }
}

struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(states::game::setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(states::game::teardown),
            );
    }
}
