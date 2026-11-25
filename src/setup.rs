use bevy::prelude::*;

use crate::*;

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub(crate) fn setup_menu(mut commands: Commands) {    
    commands.spawn((
        DespawnOnExit(AppState::MainMenu),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new("Upong"),
                TextFont {
                    font_size: TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::WHITE)
            ),
            (
                setup_button("START GAME"),
                observe(
                    |_: On<Activate>, mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::DifficultySelection);
                    }
                )
            ),
            (
                setup_button("EXIT"),
                observe(|_: On<Activate>, mut exit: MessageWriter<AppExit>| {
                    exit.write(AppExit::Success);
                })
            )
        ],
    ));
}

pub(crate) fn setup_difficulty_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::DifficultySelection),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new("SELECT DIFFICULTY"),
                TextFont {
                    font_size: TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::WHITE)
            ),
            (
                setup_button("EASY"),
                observe(
                    |_: On<Activate>,
                     mut selected: ResMut<SelectedDifficulty>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        selected.0 = Difficulty::Easy;
                        next_state.set(AppState::InGame);
                        // println!("Selected '{selected:#?}' difficulty");
                    }
                )
            ),
            (
                setup_button("MEDIUM"),
                observe(
                    |_: On<Activate>,
                     mut selected: ResMut<SelectedDifficulty>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        selected.0 = Difficulty::Medium;
                        next_state.set(AppState::InGame);
                        // println!("Selected '{selected:#?}' difficulty");
                    }
                )
            ),
            (
                setup_button("HARD"),
                observe(
                    |_: On<Activate>,
                     mut selected: ResMut<SelectedDifficulty>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        selected.0 = Difficulty::Hard;
                        next_state.set(AppState::InGame);
                        // println!("Selected '{selected:#?}' difficulty");
                    }
                )
            )
        ],
    ));
}

pub(crate) fn setup_button(label: &'static str) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            width: px(240),
            height: px(60),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        BorderColor::all(Color::WHITE),
        children![(
            Text::new(label),
            TextFont {
                font_size: TEXT_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE)
        )],
    )
}

pub(crate) fn setup_ingame_objects(
    mut commands: Commands,
    difficulty: Res<SelectedDifficulty>,
) {

    // Won/lost messages
    commands.spawn((
        DespawnOnExit(AppState::InGame),
        GameStatusText,
        Text2d::new(""),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(
            MESSAGE_X_POSITION,
            MESSAGE_Y_POSITION,
            0.0
        )
    ));

    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Score,
        EnemyScoreText,
        Text2d::new("0"),
        TextFont {
            font_size: SCORE_FONT_SIZE,
            font_features: vec![FontFeatureTag::TABULAR_FIGURES].into(),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_xyz(
            -SCORE_X_POSITION, 
            SCORE_Y_POSITION, 
            0.0
        ),
    ));

    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Score,
        PlayerScoreText,
        Text2d::new("0"),
        TextFont {
            font_size: SCORE_FONT_SIZE,
            font_features: vec![FontFeatureTag::TABULAR_FIGURES].into(),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_xyz(
            SCORE_X_POSITION, 
            SCORE_Y_POSITION,
            0.0
        ),
    ));

    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Object,
        Ball {
            velocity: Vec2::new(difficulty.ball_speed_x(), -difficulty.ball_speed_y()),
        },
        Sprite::from_color(
            Color::WHITE, 
            Vec2::splat(BALL_SIZE),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Object,
        Player,
        Paddle {
            speed: difficulty.player_paddle_speed()
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
        ),
        Transform::from_xyz(
            PLAYER_POSITION,
            0.0,
            0.0
        )
    ));
    
    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Object,
        Enemy,
        Paddle {
            speed: difficulty.enemy_paddle_speed()
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
        ),
        Transform::from_xyz(
            ENEMY_POSITION, 
            0.0,
            0.0
        )
    ));
}