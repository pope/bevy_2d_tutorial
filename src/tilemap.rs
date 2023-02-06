use crate::{ascii::AsciiSheet, GameState, TILE_SIZE};
use bevy::prelude::*;
use indoc::indoc;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct EncounterSpawner;

pub struct TileMapPluging;

impl Plugin for TileMapPluging {
	fn build(&self, app: &mut App) {
		app.add_system_set(
			SystemSet::on_enter(GameState::Overworld).with_system(show_map),
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Overworld).with_system(hide_map),
		)
		.add_startup_system(create_simple_map);
	}
}

fn show_map(mut visibility: Query<&mut Visibility, With<Map>>) {
	let mut visibility = visibility.single_mut();
	visibility.is_visible = true;
}

fn hide_map(mut visibility: Query<&mut Visibility, With<Map>>) {
	let mut visibility = visibility.single_mut();
	visibility.is_visible = false;
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
	let map = indoc! {"
		##############
		#....~~~~~~..#
		#....~~~~~~..#
		#....######..#
		#....#....#..#
		#.........#..#
		##############
	"};

	let tiles: Vec<_> = map
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.chars().enumerate().map(move |(x, char)| (y, x, char))
		})
		.map(|(y, x, char)| {
			let entity = commands
				.spawn(SpriteSheetBundle {
					sprite: TextureAtlasSprite {
						index: char as usize,
						color: Color::rgb(0.9, 0.9, 0.9),
						custom_size: Some(Vec2::splat(TILE_SIZE)),
						..default()
					},
					texture_atlas: ascii.0.clone(),
					transform: Transform {
						translation: Vec3::new(
							x as f32 * TILE_SIZE,
							-(y as f32) * TILE_SIZE,
							2.0,
						),
						..default()
					},
					..default()
				})
				.insert(Name::from(format!("Tile ({y}, {x})")))
				.id();
			match char {
				'#' => {
					commands.entity(entity).insert(TileCollider);
				}
				'~' => {
					commands.entity(entity).insert(EncounterSpawner);
				}
				_ => {}
			};
			entity
		})
		.collect();

	commands
		.spawn(SpatialBundle::default())
		.insert(Name::new("Map"))
		.insert(Map)
		.push_children(&tiles);
}
