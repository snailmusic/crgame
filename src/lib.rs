mod level_loader;

use bevy::prelude::*;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::*, Actionlike, InputManagerBundle};
use level_loader::{level::Level, LevelLoader};
use serde::{Deserialize, Serialize};


const KEY_POS: f32 = -300.;

const KEY_WIDTH: f32 = 64.;
const KEY_HEIGHT: f32 = 102.;

const SCROLL_SPEED: f32 = 800.;

const START_DELAY: f32 = 2.;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Actions>::default())
        .init_resource::<LevelState>()
        .insert_resource(StartTimer(Timer::from_seconds(START_DELAY, TimerMode::Once)))
        .init_asset::<Level>()
        .init_asset_loader::<LevelLoader>()
        .add_systems(Startup, (setup, start_level_load).chain())
        .add_systems(Update, (update_notes, check_input, load_level).chain())
        .run();
}

#[derive(Component, Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub enum Actions {
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

#[derive(Resource)]
struct StartTimer(Timer);

#[derive(Component)]
struct KeyUI;

#[derive(Component)]
struct NoteUI;

#[derive(Component)]
struct NoteTime(u32);

#[derive(Bundle)]
struct NoteBundle {
    sprite_bundle: SpriteBundle,
    action: Actions,
    time: NoteTime,
    note: NoteUI
}

impl NoteBundle {
    fn new(action: Actions, asset_server: &Res<AssetServer>, time: u32) -> Self {
        let texture = match action {
            Actions::Key1 | Actions::Key4 => "Note_A.png",
            Actions::Key2 | Actions::Key3 => "Note_B.png"
        };
        let time = time + (START_DELAY * 1000.) as u32;
        let y_pos = time_to_pos(time);
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(texture),
                transform: Transform { translation: Vec3 { x: action.get_pos(), y: y_pos, z: 0.0 }, ..default() },
                ..default()
            },
            action: action,
            time: NoteTime(time),
            note: NoteUI
        }
    }
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

#[derive(Resource,Default)]
struct LevelState {
    level_handle: Handle<Level>,
    songname: Option<String>,
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

fn start_level_load(mut level_state: ResMut<LevelState>, asset_server: Res<AssetServer>, ) {
    level_state.level_handle = asset_server.load("levels/level.yml");
}

fn check_input(
    query: Query<&ActionState<Actions>>,
    mut keys: Query<(&mut Transform, &Actions), With<KeyUI>>
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

fn time_to_pos(time: u32) -> f32 {
    (time as f32 / 1000.) * SCROLL_SPEED  + KEY_POS + KEY_HEIGHT * 0.5
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_state: ResMut<LevelState>,
    level_assets: Res<Assets<Level>>,
    mut timer: ResMut<StartTimer>,
    time: Res<Time>,
) {

    if !level_state.songname.is_none() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.spawn(AudioBundle {
                source: asset_server.load(&("levels/".to_string() + &level_state.songname.clone().unwrap())),
                settings: PlaybackSettings::DESPAWN
            });
        }
        return;
    }

    let level_asset = level_assets.get(&level_state.level_handle);

    if level_asset.is_none() {
        info!("still loading level...");
        return;
    }

    let level_asset = level_asset.unwrap();

    info!("Loaded {:?} in {:?}s", level_asset, time.elapsed_seconds());

    
    for note in level_asset.data.iter() {
        match note {
            level_loader::level::Note::S(action, time) => {
                commands.spawn(NoteBundle::new(*action, &asset_server, *time));
            },
            level_loader::level::Note::L(_, _, _) => todo!(),
        }
    }

    level_state.songname = Some(level_asset.filename.clone());
}

fn update_notes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut NoteTime), With<NoteUI>>,
    time: Res<Time>
) {
    for (entity, mut trans, mut notetime) in query.iter_mut() {
        if notetime.0 < (time.delta_seconds() * 1000.).round() as u32 {
            commands.entity(entity).despawn();
        }
        else {
            notetime.0 -= (time.delta_seconds() * 1000.).round() as u32;
            trans.translation.y = time_to_pos(notetime.0);
        }
    }
}