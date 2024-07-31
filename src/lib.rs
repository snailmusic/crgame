use bevy::prelude::*;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::*, Actionlike, InputManagerBundle};

const KEY_LIST: [Actions;4] = [
    Actions::Key1,
    Actions::Key2,
    Actions::Key3,
    Actions::Key4
];

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Actions>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, check_input)
        .run();
}

#[derive(Component, Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Actions {
    Key1,
    Key2,
    Key3,
    Key4
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let input_map = InputMap::new([
        (Actions::Key1, KeyCode::KeyD),
        (Actions::Key2, KeyCode::KeyF),
        (Actions::Key3, KeyCode::KeyJ),
        (Actions::Key4, KeyCode::KeyK),
        ]);
    
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(InputManagerBundle::with_map(input_map));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Key_A.png"),
            transform: Transform { translation: Vec3 { x: -128., y: -200., z: 0.0 }, ..default() },
            ..default()
        },
        Actions::Key1
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Key_B.png"),
            transform: Transform { translation: Vec3 { x: -64., y: -200., z: 0.0 }, ..default() },
            ..default()
        },
        Actions::Key2
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Key_B.png"),
            transform: Transform { translation: Vec3 { x: 0., y: -200., z: 0.0 }, ..default() },
            ..default()
        },
        Actions::Key3
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Key_A.png"),
            transform: Transform { translation: Vec3 { x: 64., y: -200., z: 0.0 }, ..default() },
            ..default()
        },
        Actions::Key4
    ));
}

fn check_input(
    query: Query<&ActionState<Actions>>,
    mut keys: Query<(&mut Transform, &Actions)>
) {
    let action_state = query.single();
    
    for (mut trans, key) in keys.iter_mut() {
        if action_state.pressed(key) {
            println!("yippee!! {:?}", key);
            trans.scale = Vec3::new(0.9, 0.9, 1.);
        }
        else {
            trans.scale = Vec3::new(1., 1., 1.);
        }
    }
}