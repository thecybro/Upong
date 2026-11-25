mod diff;
mod utils;
mod setup;
mod game;
mod movement;
mod collision;

use setup::*;
use game::*;
use movement::*;
use collision::*;

use bevy::{
    picking::hover::Hovered,
    prelude::*,
    sprite::Anchor,
    text::FontFeatureTag,
    ui::Pressed,
    ui_widgets::{Activate, Button, observe},
};

const WINDOW_WIDTH: f32 = 650.0;
const WINDOW_HEIGHT: f32 = 450.0;
// diagonal when (w, h) = (650, 450) is 790.5

const BALL_SIZE: f32 = 15.0;

const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = WINDOW_HEIGHT / 6.0;

const PADDLE_MAX_Y: f32 = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;

const BALL_MAX_X: f32 = WINDOW_WIDTH / 2.0 - BALL_SIZE / 2.0;
const BALL_MAX_Y: f32 = WINDOW_HEIGHT / 2.0 - BALL_SIZE / 2.0;

const PLAYER_POSITION: f32 = (WINDOW_WIDTH - PADDLE_WIDTH) / 2.0;
const ENEMY_POSITION: f32 = (-WINDOW_WIDTH + PADDLE_WIDTH) / 2.0;

const REFRESH_RATE: f64 = 60.0;

const MAX_BOUNCE_ANGLE: f32 = 45.0_f32.to_radians();

const TEXT_FONT_SIZE: FontSize = FontSize::Px(32.0);
const SCORE_FONT_SIZE: FontSize = FontSize::Px(42.0);

const SCORE_X_POSITION: f32 = WINDOW_WIDTH / 10.0;
const SCORE_Y_POSITION: f32 = WINDOW_HEIGHT / 2.5;

const MAX_SCORE: u32 = 10;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}
#[derive(Component)]
struct Paddle {
    speed: f32,
}
#[derive(Component)] struct Player;

#[derive(Component)] struct Enemy;

#[derive(Debug, Resource, PartialEq)]
enum GamePhase {
    Waiting,
    Playing,
    Won,
    Lost,
}

#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    MainMenu,
    DifficultySelection,
    InGame,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Resource, Debug)]
struct SelectedDifficulty(Difficulty);

#[derive(Component)] struct GameStatusText;

#[derive(Resource, Debug, Default)]
struct GameScores {
    enemy: u32,
    player: u32,
}

#[derive(Component)] struct Object;

/// Marker component for scores
#[derive(Component)] struct Score;

#[derive(Component)] struct PlayerScoreText;
#[derive(Component)] struct EnemyScoreText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Upong".to_string(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(REFRESH_RATE))
        .init_state::<AppState>()
        .insert_resource(GamePhase::Waiting)
        .insert_resource(GameScores::default())
        .insert_resource(SelectedDifficulty(Difficulty::Easy))
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(AppState::MainMenu), 
            setup_menu
        )
        .add_systems(
            OnEnter(AppState::DifficultySelection),
            setup_difficulty_menu,
        )
        .add_systems(
            Update, 
            restart_game.run_if(is_over)
        )
        .add_systems(
            OnEnter(AppState::InGame),
            (enter_game, setup_ingame_objects),
        )
        .add_systems(
            Update,
            (
                // if the user has lost/won, this puts them in the main menu again
                // if user clicked Enter
                reset_game_objects, 
                update_button_appearance,
                update_game_scores,
                check_game_over,
                update_game_status_texts,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                // We want to handle these operations in order
                player_movement,
                ball_movement,
                enemy_movement,
                paddle_collisions,
            )
                // To handle these operations in order, we use .chain()
                .chain()
                .run_if(in_state(AppState::InGame))
                // Those operations will run only if this returns true
                // The condition here is evaluated for the entire chain;
                // which doesn't happen after every system call,
                // It happens after the FixedUpdate system has finished
                // And FixedUpdate system runs as a whole if this evaluates to true
                .run_if(is_playing),
        )
        .run();
}


fn update_button_appearance(
    buttons: Query<(&Hovered, Has<Pressed>, &mut BackgroundColor), With<Button>>,
) {
    for (hovered, pressed, mut color) in buttons {
        *color = match (hovered.get(), pressed) {
            (_, true) => PRESSED_BUTTON.into(),
            (true, false) => HOVERED_BUTTON.into(),
            (false, false) => NORMAL_BUTTON.into(),
        }
    }
}
