use bevy::prelude::*;

use crate::*;

const WON_MESSAGE: &'static str = "YOU WON!\nPress Enter to go back to menu";
const LOST_MESSAGE: &'static str = "You lost!\nPress Enter to go back to menu";

pub(crate) const MESSAGE_X_POSITION: f32 = 0.0;
pub(crate) const MESSAGE_Y_POSITION: f32 = WINDOW_HEIGHT / 4.0;

pub(crate) fn enter_game(
    mut phase: ResMut<GamePhase>,
    // selected: Res<SelectedDifficulty>,
) {
    *phase = GamePhase::Playing;

    println!("Entering game!");
    // println!("Selected difficulty: {selected:#?}");
}

/// If the user has lost/won, bring them back to main menu;
/// If they clicked Enter
pub(crate) fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut phase: ResMut<GamePhase>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter)
    && 
    matches!(*phase, GamePhase::Won | GamePhase::Lost) {
        next_state.set(AppState::MainMenu);
        *phase = GamePhase::Waiting;
    } 
}

/// Reset the paddles & ball back to default if the phase has changed
/// and the phase is playing; which happens only when user explicitly
/// presses Enter
///
/// Phase doesn't change to Playing without user pressing Enter
pub(crate) fn reset_game_objects(
    phase: Res<GamePhase>,
    difficulty: Res<SelectedDifficulty>,
    mut scores: ResMut<GameScores>,
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Ball>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Ball>, Without<Player>)>,
) {
    // If the phase was just changed and current phase is Playing,
    // then only we want to reset the game.
    //
    // Player can transition from Waiting/Won/Lost to Playing by
    // pressing Enter.
    // So whenever that happens; we reset the objects back to default
    if !phase.is_changed() || *phase != GamePhase::Playing {
        return;
    }

    scores.enemy = 0;
    scores.player = 0;

    for (mut ball, mut ball_transform) in &mut ball_query {
        ball_transform.translation = Vec3::ZERO;
        ball.velocity = Vec2::new(difficulty.ball_speed_x(), -difficulty.ball_speed_y());
    }

    for mut player_transform in &mut player_query {
        player_transform.translation = Vec3::new(PLAYER_POSITION, 0.0, 0.0);
    }

    for mut enemy_transform in &mut enemy_query {
        enemy_transform.translation = Vec3::new(ENEMY_POSITION, 0.0, 0.0);
    }
}

/// Let users select menu option by pressing keys as well
pub(crate) fn select_menu_option_by_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::DifficultySelection);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

/// Let users select difficulty by key as well
pub(crate) fn select_difficulty_by_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedDifficulty>,
    mut next_state: ResMut<NextState<AppState>>
) {
    let difficulty = if keyboard.just_pressed(KeyCode::KeyE) {
        Some(Difficulty::Easy)
    } else if keyboard.just_pressed(KeyCode::KeyM) {
        Some(Difficulty::Medium)
    } else if keyboard.just_pressed(KeyCode::KeyH) {
        Some(Difficulty::Hard)
    } else {
        None
    };

    if let Some(diff) = difficulty {
        selected.0 = diff;
        next_state.set(AppState::InGame);
    }
}

pub(crate) fn is_playing(phase: Res<GamePhase>) -> bool {
    *phase == GamePhase::Playing
}

pub(crate) fn is_over(phase: Res<GamePhase>) -> bool {
    matches!(*phase, GamePhase::Won | GamePhase::Lost)
}

/// This increases the score if the score has increased somehow
/// It doesn't care if someone/something increased score even outside
/// of Playing game phase
pub(crate) fn update_game_scores(
    scores: Res<GameScores>,
    mut player_score_query: Query<&mut Text2d, (With<PlayerScoreText>, Without<EnemyScoreText>)>,
    mut enemy_score_query: Query<&mut Text2d, (With<EnemyScoreText>, Without<PlayerScoreText>)>,
) {
    if !scores.is_changed() {
        return;
    }

    if let Ok(mut text) = player_score_query.single_mut() {
        **text = scores.player.to_string();
    }

    if let Ok(mut text) = enemy_score_query.single_mut() {
        **text = scores.enemy.to_string();
    }
}


/// This declares that player has won/lost based on whose score
/// has reached max score
pub(crate) fn check_game_over(
    mut phase: ResMut<GamePhase>, 
    scores: Res<GameScores>
) {
    // Player either wins or loses only when either has reached max score
    if scores.enemy >= MAX_SCORE {
        *phase = GamePhase::Lost;
    } else if scores.player >= MAX_SCORE {
        *phase = GamePhase::Won;
    }
}

pub(crate) fn update_game_status_texts(
    phase: Res<GamePhase>,
    mut status_text: Query<&mut Text2d, With<GameStatusText>>
    
) {
    // println!("Got phase: {phase:#?}");
    
    let message = match *phase {
        GamePhase::Won => WON_MESSAGE,
        GamePhase::Lost => LOST_MESSAGE,
        _ => ""
    };

    // println!("Finalized the message {message}");
    for mut text in &mut status_text {
        // println!("Got text {text:#?}");
        if text.0 != message {
            text.0 = message.to_string();
        }
        // println!("Set the text to {text:#?}");
    }
}