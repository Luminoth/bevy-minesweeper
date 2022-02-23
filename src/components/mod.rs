use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct MainCamera;

#[derive(Debug, Default, Component, Inspectable)]
pub struct UiCamera;
