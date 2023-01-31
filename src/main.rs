use bevy::{prelude::*, render::camera::ScalingMode};

mod ascii;
mod debug;
mod player;
mod tilemap;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use tilemap::TileMapPluging;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
	App::new()
		.insert_resource(ClearColor(CLEAR))
		.add_startup_system(spawn_camera)
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(
			WindowPlugin {
				window: WindowDescriptor {
					width: HEIGHT * ASPECT_RATIO,
					height: HEIGHT,
					resizable: false,
					title: "Bevy 2D Tutorial".to_string(),
					..default()
				},
				..default()
			},
		))
		.add_plugin(AsciiPlugin)
		.add_plugin(DebugPlugin)
		.add_plugin(PlayerPlugin)
		.add_plugin(TileMapPluging)
		.run();
}

fn spawn_camera(mut commands: Commands) {
	let camera = Camera2dBundle {
		projection: OrthographicProjection {
			top: 1.0,
			bottom: -1.0,
			right: 1.0 * ASPECT_RATIO,
			left: -1.0 * ASPECT_RATIO,
			scaling_mode: ScalingMode::None,
			..default()
		},
		..default()
	};
	commands.spawn(camera);
}
