use crate::{ascii::AsciiSheet, tilemap::TileCollider, TILE_SIZE};
use bevy::{prelude::*, sprite::collide_aabb::collide};

type WallQuery<'w, 's, 'a> =
	Query<'w, 's, (&'a Transform, (With<TileCollider>, Without<Player>))>;

#[derive(Component, Reflect)]
pub struct Player {
	speed: f32,
}

impl Default for Player {
	fn default() -> Self {
		Player { speed: 1.0 }
	}
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(spawn_player)
			.add_system(player_movement.label("movement"))
			.add_system(camera_follow.after("movement"));
	}
}

fn camera_follow(
	player_query: Query<&Transform, With<Player>>,
	mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
	let player_transform = player_query.single();
	let mut camera_transform = camera_query.single_mut();

	camera_transform.translation.x = player_transform.translation.x;
	camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
	mut player_query: Query<(&Player, &mut Transform)>,
	wall_query: WallQuery,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (player, mut transform) = player_query.single_mut();

	let speed = player.speed * TILE_SIZE * time.delta_seconds();

	let mut y_delta = 0.0;
	if keyboard.pressed(KeyCode::W) {
		y_delta += speed;
	}
	if keyboard.pressed(KeyCode::S) {
		y_delta -= speed;
	}

	let mut x_delta = 0.0;
	if keyboard.pressed(KeyCode::A) {
		x_delta -= speed;
	}
	if keyboard.pressed(KeyCode::D) {
		x_delta += speed;
	}

	let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
	if wall_collision_check(target, &wall_query) {
		transform.translation = target;
	}

	let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
	if wall_collision_check(target, &wall_query) {
		transform.translation = target;
	}
}

fn wall_collision_check(
	target_player_pos: Vec3,
	wall_query: &WallQuery,
) -> bool {
	for wall_transform in wall_query.iter() {
		let collision = collide(
			target_player_pos,
			Vec2::splat(TILE_SIZE * 0.9),
			wall_transform.0.translation,
			Vec2::splat(TILE_SIZE),
		);
		if collision.is_some() {
			return false;
		}
	}
	true
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
	let player_sprite = TextureAtlasSprite {
		index: 1,
		color: Color::rgb(0.3, 0.3, 0.9),
		custom_size: Some(Vec2::splat(TILE_SIZE)),
		..default()
	};

	let player = commands
		.spawn(SpriteSheetBundle {
			sprite: player_sprite,
			texture_atlas: ascii.0.clone(),
			transform: Transform {
				translation: Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 9.0),
				..default()
			},
			..default()
		})
		.insert(Name::new("Player"))
		.insert(Player { speed: 3.0 })
		.id();

	let background_sprite = TextureAtlasSprite {
		index: 0,
		color: Color::rgb(0.5, 0.5, 0.5),
		custom_size: Some(Vec2::splat(TILE_SIZE)),
		..default()
	};

	let background = commands
		.spawn(SpriteSheetBundle {
			sprite: background_sprite,
			texture_atlas: ascii.0.clone(),
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, -1.0),
				..default()
			},
			..default()
		})
		.insert(Name::new("Background"))
		.id();

	commands.entity(player).add_child(background);
}