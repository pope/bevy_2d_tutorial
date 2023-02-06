use crate::{
	combat::Enemy,
	player::{EncounterTimer, Player},
};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		if cfg!(debug_assertions) {
			app.add_plugin(WorldInspectorPlugin)
				.init_resource::<EncounterTimer>()
				.register_type::<EncounterTimer>()
				.register_type::<Player>()
				.register_type::<Enemy>();
		}
	}
}
