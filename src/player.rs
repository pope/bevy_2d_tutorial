use crate::{
	ascii::AsciiSheet,
	tilemap::{EncounterSpawner, TileCollider},
	GameState, TILE_SIZE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

type NonPlayerTransformQuery<'w, 's, 'a, T> =
	Query<'w, 's, (&'a Transform, (With<T>, Without<Player>))>;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
	speed: f32,
	just_moved: bool,
}

impl Default for Player {
	fn default() -> Self {
		Player {
			speed: 1.0,
			just_moved: false,
		}
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct EncounterTimer {
	timer: Timer,
}

impl Default for EncounterTimer {
	fn default() -> Self {
		EncounterTimer {
			timer: Timer::from_seconds(1.0, TimerMode::Repeating),
		}
	}
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(EncounterTimer::default())
			.add_system_set(
				SystemSet::on_enter(GameState::Overworld)
					.with_system(show_player),
			)
			.add_system_set(
				SystemSet::on_exit(GameState::Overworld)
					.with_system(hide_player),
			)
			.add_system_set(
				SystemSet::on_update(GameState::Overworld)
					.with_system(player_movement.label("movement"))
					.with_system(player_encounter_checking.after("movement"))
					.with_system(camera_follow.after("movement")),
			)
			.add_startup_system(spawn_player);
	}
}

fn hide_player(mut player_visibility: Query<&mut Visibility, With<Player>>) {
	set_player_visibility(&mut player_visibility, false);
}

fn show_player(mut player_visibility: Query<&mut Visibility, With<Player>>) {
	set_player_visibility(&mut player_visibility, true);
}

fn set_player_visibility(
	player_visibility: &mut Query<&mut Visibility, With<Player>>,
	is_visible: bool,
) {
	let mut player_visibility = player_visibility.single_mut();
	player_visibility.is_visible = is_visible;
}

fn player_encounter_checking(
	player_query: Query<(&Player, &Transform)>,
	encounter_query: NonPlayerTransformQuery<EncounterSpawner>,
	time: Res<Time>,
	mut encounter_timer: ResMut<EncounterTimer>,
	mut state: ResMut<State<GameState>>,
) {
	let (player, transform) = player_query.single();

	if player.just_moved
		&& player_collides(transform.translation, &encounter_query)
	{
		let timer = encounter_timer.timer.tick(time.delta());
		if timer.just_finished() {
			println!("You're in the danger zone!");
			state
				.set(GameState::Combat)
				.expect("Failed to change states");
		}
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
	mut player_query: Query<(&mut Player, &mut Transform)>,
	wall_query: NonPlayerTransformQuery<TileCollider>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (mut player, mut transform) = player_query.single_mut();
	player.just_moved = false;

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

	if x_delta == 0.0 && y_delta == 0.0 {
		return;
	}

	player.just_moved = true;

	let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
	if !player_collides(target, &wall_query) {
		transform.translation = target;
	}

	let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
	if !player_collides(target, &wall_query) {
		transform.translation = target;
	}
}

fn player_collides<C: Component>(
	target_player_pos: Vec3,
	query: &NonPlayerTransformQuery<C>,
) -> bool {
	query.iter().any(|transform| {
		collide(
			target_player_pos,
			Vec2::splat(TILE_SIZE * 0.9),
			transform.0.translation,
			Vec2::splat(TILE_SIZE),
		)
		.is_some()
	})
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
		.insert(Player {
			speed: 3.0,
			..default()
		})
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
