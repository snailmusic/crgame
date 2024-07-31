use bevy::prelude::*;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::*, Actionlike, InputManagerBundle};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Actions>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, check_input)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Actions {
    Key1,
    Key2,
    Key3,
    Key4
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (Actions::Key1, KeyCode::KeyD),
        (Actions::Key2, KeyCode::KeyF),
        (Actions::Key3, KeyCode::KeyJ),
        (Actions::Key4, KeyCode::KeyK),
        ]);
    
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(InputManagerBundle::with_map(input_map));
}

fn check_input(
    query: Query<&ActionState<Actions>>
) {
    let action_state = query.single();
    if action_state.just_pressed(&Actions::Key1) {
        println!("1");
    }

    if action_state.just_pressed(&Actions::Key2) {
        println!("2");
    }

    if action_state.just_pressed(&Actions::Key3) {
        println!("3");
    }

    if action_state.just_pressed(&Actions::Key4) {
        println!("4");
    }
}