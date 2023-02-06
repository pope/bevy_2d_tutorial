use crate::{ascii::AsciiSheet, GameState, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(
			SystemSet::on_update(GameState::Combat)
				.with_system(test_exit_combat),
		)
		.add_system_set(
			SystemSet::on_enter(GameState::Combat)
				.with_system(spawn_enemy)
				.with_system(combat_camera),
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy),
		);
	}
}

fn test_exit_combat(
	mut keyboard: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
) {
	if keyboard.just_pressed(KeyCode::Space) {
		println!("Let's go back!");
		state
			.set(GameState::Overworld)
			.expect("Failed to change state");
		keyboard.clear();
	}
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSheet>) {
	let sprite = SpriteSheetBundle {
		sprite: TextureAtlasSprite {
			index: 'b' as usize,
			color: Color::rgb(0.8, 0.8, 0.8),
			custom_size: Some(Vec2::splat(TILE_SIZE)),
			..default()
		},
		texture_atlas: ascii.0.clone(),
		transform: Transform {
			translation: Vec3::new(0.0, 0.5, 9.0),
			..default()
		},
		..default()
	};
	commands
		.spawn(sprite)
		.insert(Enemy)
		.insert(Name::new("Enemy"));
}

fn despawn_enemy(mut commands: Commands, enemy: Query<Entity, With<Enemy>>) {
	for entity in enemy.iter() {
		commands.entity(entity).despawn_recursive();
	}
}

fn combat_camera(mut transform: Query<&mut Transform, With<Camera>>) {
	let mut transform = transform.single_mut();
	transform.translation.x = 0.0;
	transform.translation.y = 0.0;
}
