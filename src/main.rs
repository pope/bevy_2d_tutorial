use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 720.0;

#[derive(Resource)]
struct AsciiSheet(Handle<TextureAtlas>);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: HEIGHT * ASPECT_RATIO,
                        height: HEIGHT,
                        resizable: false,
                        title: "Bevy 2D Tutorial".to_string(),
                        ..default()
                    },
                    ..default()
                }),
        )
        .run();
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player_sprite = TextureAtlasSprite {
        index: 1,
        color: Color::rgb(0.3, 0.3, 0.9),
        custom_size: Some(Vec2::splat(1.0)),
        ..default()
    };

    let player = commands
        .spawn(SpriteSheetBundle {
            sprite: player_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 9.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .id();

    let background_sprite = TextureAtlasSprite {
        index: 0,
        color: Color::rgb(0.5, 0.5, 0.5),
        custom_size: Some(Vec2::splat(1.0)),
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
