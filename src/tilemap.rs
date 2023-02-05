use crate::{ascii::AsciiSheet, TILE_SIZE};
use bevy::prelude::*;
use indoc::indoc;

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPluging;

impl Plugin for TileMapPluging {
	fn build(&self, app: &mut App) {
		app.add_startup_system(create_simple_map);
	}
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
	let map = indoc! {"
		##############
		#......#.....#
		#............#
		#....######..#
		#....#....#..#
		#.........#..#
		##############
	"};

	let tiles: Vec<_> = map
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			// TODO(pope): Figure out how to avoid the collect allocation.
			// Maybe the most efficient thing to do here is a nested for-loop
			//
			// Error without collect:
			//     `returns a reference to data owned by the current function`
			line.chars()
				.enumerate()
				.map(|(x, char)| (y, x, char))
				.collect::<Vec<_>>()
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
			if char == '#' {
				commands.entity(entity).insert(TileCollider);
			}
			entity
		})
		.collect();

	commands
		.spawn(SpatialBundle::default())
		.insert(Name::new("Map"))
		.push_children(&tiles);
}
