use bevy::prelude::*;

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
	}
}

fn load_ascii(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle = asset_server.load("Ascii.png");
	let texture_atlas = TextureAtlas::from_grid(
		texture_handle,
		Vec2::splat(9.0),
		16,
		16,
		Some(Vec2::splat(2.0)),
		None,
	);

	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands.insert_resource(AsciiSheet(texture_atlas_handle));
}
