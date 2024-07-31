use bevy::prelude::*;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::*, Actionlike, InputManagerBundle};

const KEY_LIST: [Actions;4] = [
    Actions::Key1,
    Actions::Key2,
    Actions::Key3,
    Actions::Key4
];

const KEY_WIDTH: f32 = 64.;

const SCROLL_SPEED: f32 = 100.;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Actions>::default())
        .add_systems(Startup, (setup, add_notes).chain())
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

impl Actions {
    fn get_pos(&self) -> f32 {
        match self {
            Actions::Key1 => -KEY_WIDTH * 1.5,
            Actions::Key2 => -KEY_WIDTH * 0.5,
            Actions::Key3 => KEY_WIDTH * 0.5,
            Actions::Key4 => KEY_WIDTH * 1.5,
        }
    }
}

#[derive(Component)]
struct KeyUI;

#[derive(Component)]
struct NoteUI;


#[derive(Bundle)]
struct NoteBundle {
    sprite_bundle: SpriteBundle,
    note: NoteUI
}

impl NoteBundle {

}

#[derive(Bundle)]
struct KeyBundle {
    sprite_bundle: SpriteBundle,
    action: Actions,
    key: KeyUI
}

impl KeyBundle {
    fn new(action: Actions, asset_server: &Res<AssetServer>) -> Self {
        let texture = match action {
            Actions::Key1 | Actions::Key4 => "Key_A.png",
            Actions::Key2 | Actions::Key3 => "Key_B.png"
        };
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(texture),
                transform: Transform { translation: Vec3 { x: action.get_pos(), y: -300., z: 0.0 }, ..default() },
                ..default()
            },
            action: action,
            key: KeyUI
        }
    }
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

    commands.spawn(KeyBundle::new(Actions::Key1, &asset_server));
    commands.spawn(KeyBundle::new(Actions::Key2, &asset_server));
    commands.spawn(KeyBundle::new(Actions::Key3, &asset_server));
    commands.spawn(KeyBundle::new(Actions::Key4, &asset_server));
}

fn add_notes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let notea_handle = asset_server.load("Note_A.png");
    let noteb_handle: Handle<Image> = asset_server.load("Note_B.png");
    commands.spawn(
        SpriteBundle{
            texture: notea_handle.clone(),
            ..default()
        }
    );
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