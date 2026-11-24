mod utils;

use bevy::{prelude::*, sprite::Anchor, text::FontFeatureTag};

const WINDOW_WIDTH: f32 = 650.0;
const WINDOW_HEIGHT: f32 = 450.0; 
// diagonal when (w, h) = (650, 450) is 790.5

const BALL_SIZE: f32 = 15.0;

const BALL_SPEED_X: f32 = 300.0; 
const BALL_SPEED_Y: f32 = 300.0; 

const BALL_SPEED: f32 = utils::const_sqrt(
    (BALL_SPEED_X * BALL_SPEED_X) 
    + 
    (BALL_SPEED_Y * BALL_SPEED_Y)
    );

const BALL_MAX_SPEED_X: f32 = BALL_SPEED_X * 2.0;
const BALL_MAX_SPEED_Y: f32 = BALL_SPEED_Y * 2.0;

// This max speed is the result of plotting increased speed on both axis
// It's not the speed on each axis
const BALL_MAX_SPEED: f32 = utils::const_sqrt(
    (BALL_MAX_SPEED_X * BALL_MAX_SPEED_X) 
    + 
    (BALL_MAX_SPEED_Y * BALL_MAX_SPEED_Y)
    );

const BALL_SPEED_INCREASE: f32 = 0.3;
const BALL_SPEED_DECREASE: f32 = 1.02;

const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = WINDOW_HEIGHT / 6.0;
const PLAYER_PADDLE_SPEED: f32 = 450.0;

// Make enemy speed 80% of player cuz it would be too unfair otherwise
const ENEMY_PADDLE_SPEED: f32 = PLAYER_PADDLE_SPEED * 0.8;
const ENEMY_DISTANCE: f32 = 10.0; // 10 pixels

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

const WAITING_MESSAGE: &'static str = "UPONG\nPress Enter to start";
const WON_MESSAGE: &'static str = "YOU WON!\nPress Enter to play again";
const LOST_MESSAGE: &'static str = "You lost!\nPress Enter to try again";

#[derive(Component)] struct Ball { velocity: Vec2 }
#[derive(Component)] struct Paddle { speed: f32 }
#[derive(Component)] struct Player;
#[derive(Component)] struct Enemy;

#[derive(Resource, PartialEq)]
enum GamePhase {
    Waiting,
    Playing,
    Won,
    Lost,
}

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
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            title: "Upong".to_string(),
                            resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                            resizable: false,
                            ..default()
                        }
                    ),
                    ..default()
                }
            )
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(REFRESH_RATE))
        .insert_resource(GamePhase::Waiting)
        .insert_resource(GameScores::default())
        .add_systems(Startup, setup)
        .add_systems(Update, 
            (
                start_game, 
                reset_game, 
                update_game_visibility, 
                update_game_scores, 
                update_game_status
            ).chain()
        )
        .add_systems(
            FixedUpdate, 
            (
                // We want to handle these operations in order
                player_movement, 
                ball_movement,
                enemy_movement,
                paddle_collisions,
                check_game_over
            )
            // To handle these operations in order, we use .chain()
            .chain()

            // Those operations will run only if this returns true
            // The condition here is evaluated for the entire chain;
            // which doesn't happen after every system call,
            // It happens after the FixedUpdate system has finished
            // And FixedUpdate system runs as a whole if this evaluates to true
            .run_if(is_playing),
        )
        .run();
}

/// Setup just sets up the initial window/page visible to players, and
/// when they switch to Playing state, then only they see paddles and balls
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // The initial text that appears on screen
    commands.spawn(Node {
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(
            (
                GameStatusText,
                Text::new(WAITING_MESSAGE.to_string()),
                TextFont {
                    font_size: TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::justify(Justify::Center),
            )
        );
    });

    commands.spawn((
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
        Transform::from_xyz(-SCORE_X_POSITION, SCORE_Y_POSITION, 0.0),
        Visibility::Hidden,
    ));

    commands.spawn((
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
        Transform::from_xyz(SCORE_X_POSITION, SCORE_Y_POSITION, 0.0),
        Visibility::Hidden
    ));
    
    // Ball (at center)
    commands.spawn((
        Object,
        Ball {
            velocity: Vec2::new(BALL_SPEED_X, -BALL_SPEED_Y)
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::splat(BALL_SIZE),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Hidden
    ));

    // Player paddle (at right)
    commands.spawn((
        Object,
        Player,
        Paddle {
            speed: PLAYER_PADDLE_SPEED,
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
        ),
        Transform::from_xyz(
            PLAYER_POSITION,
            0.0,
            0.0
        ),
        Visibility::Hidden
    ));

    // Enemy paddle (at left)
    commands.spawn((
        Object,
        Enemy,
        Paddle {
            speed: ENEMY_PADDLE_SPEED,
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        ),
        Transform::from_xyz(
            ENEMY_POSITION,
            0.0,
            0.0
        ),
        Visibility::Hidden
    ));
}

/// Press enter to start the game and if the game is over
/// Press enter again to restart
fn start_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut phase: ResMut<GamePhase>,
) {
    if *phase != GamePhase::Playing 
    && keyboard.just_pressed(KeyCode::Enter)
    {
        *phase = GamePhase::Playing;
    }
}

/// Reset the paddles & ball back to default if the phase has changed
/// and the phase is playing; which happens only when user explicitly 
/// presses Enter
/// 
/// Phase doesn't change to Playing without user pressing Enter
fn reset_game(
    phase: Res<GamePhase>,
    mut scores: ResMut<GameScores>,
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    mut player_query: Query<
        &mut Transform,
        (With<Player>, Without<Ball>, Without<Enemy>)
    >,
    mut enemy_query: Query<
        &mut Transform,
        (With<Enemy>, Without<Ball>, Without<Player>)
    >,
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
        ball.velocity = Vec2::new(BALL_SPEED_X, -BALL_SPEED_Y);
    }

    for mut player_transform in &mut player_query {
        player_transform.translation = Vec3::new(
            PLAYER_POSITION,
            0.0,
            0.0
        );
    }

    for mut enemy_transform in &mut enemy_query {
        enemy_transform.translation = Vec3::new(
            ENEMY_POSITION,
            0.0,
            0.0
        );
    }
}

fn is_playing(phase: Res<GamePhase>) -> bool {
    *phase == GamePhase::Playing
}

/// This makes all the components visible when we are not waiting
fn update_game_visibility(
    phase: Res<GamePhase>,
    mut score: Query<&mut Visibility, 
        (With<Score>, Without<Object>)>,
    mut object: Query<&mut Visibility, 
        (With<Object>, Without<Score>)>,
) {
    if !phase.is_changed() {
        return;
    }

    let visibility = match *phase {
        GamePhase::Waiting => Visibility::Hidden,
        _ => Visibility::Visible,
    };

    for mut object_visibility in &mut object {
        *object_visibility = visibility;
    }

    for mut score_visibility in &mut score {
        * score_visibility = visibility;
    }
}

/// This increases the score if the score has increased somehow
/// It doesn't care if someone/something increased score even outside
/// of Playing game phase
fn update_game_scores(
    scores: Res<GameScores>,
    mut player_score_query: Query<&mut Text2d,
        (With<PlayerScoreText>, Without<EnemyScoreText>)>,
    mut enemy_score_query: Query<&mut Text2d,
        (With<EnemyScoreText>, Without<PlayerScoreText>)>,
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

fn update_game_status(
    phase: Res<GamePhase>,
    mut query: Query<&mut Text, With<GameStatusText>>,
) {
    if !phase.is_changed() {
        return;
    }
    
    let message = match *phase {
        GamePhase::Waiting => WAITING_MESSAGE,
        GamePhase::Playing => "",
        GamePhase::Won => WON_MESSAGE,
        GamePhase::Lost => LOST_MESSAGE,
    };
    
    for mut text in &mut query {
        text.0 = message.to_string();
    }
}

/// This declares that player has won/lost based on whose score
/// has reached max score
fn check_game_over(
    mut phase: ResMut<GamePhase>,
    scores: Res<GameScores>,
) {
    // Player either wins or loses only when either has reached max score
    if scores.enemy >= MAX_SCORE {
        *phase = GamePhase::Lost;
    } else if scores.player >= MAX_SCORE {
        *phase = GamePhase::Won;
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut Transform), With<Player>>,
) {
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }

    if keyboard.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    for (paddle, mut transform) in &mut query {
        transform.translation.y += direction * paddle.speed * time.delta_secs();

        transform.translation.y = transform.translation.y.clamp(-PADDLE_MAX_Y, PADDLE_MAX_Y);
    }
}


fn ball_movement(
    time: Res<Time>,
    mut scores: ResMut<GameScores>,
    mut query: Query<(&mut Ball, &mut Transform), With<Ball>>,
) {
    for (mut ball, mut transform) in &mut query {
        let dt = time.delta_secs();

        transform.translation += ball.velocity.extend(0.0) * dt;
        
        // Restrict the ball from going above/below the screen
        // 
        // We introduce a little friction to reduce the speed of
        // the ball everytime it hits the wall but without letting
        // it go below its initial speed
        if transform.translation.y > BALL_MAX_Y {
            transform.translation.y = BALL_MAX_Y;
            ball.velocity.y = -(ball.velocity.y / BALL_SPEED_DECREASE);
            ball.velocity.y = ball.velocity.y.clamp(-BALL_SPEED_Y, BALL_MAX_SPEED_Y)
        } else if transform.translation.y < -BALL_MAX_Y {
            transform.translation.y = -BALL_MAX_Y;
            ball.velocity.y = -(ball.velocity.y / BALL_SPEED_DECREASE);
            ball.velocity.y = ball.velocity.y.clamp(BALL_SPEED_Y, BALL_MAX_SPEED_Y)
        }

        // When ball touches right edge (where player is)
        // Player missed
        // 
        // Here, we dont have to make the ball bounce because it's being
        // returned to original position anyway
        if transform.translation.x > BALL_MAX_X {
            scores.enemy += 1;

            // Bring back ball to default position once enemy scores
            // // But reduce y speed a bit to let player react
            transform.translation = Vec3::new(ENEMY_POSITION + 20.0, 0.0, 0.0);
            ball.velocity = Vec2::new(BALL_SPEED_X * ( 1.0 + BALL_SPEED_INCREASE), 0.0);
        }
        // When ball touches left edge (where enemy is)
        // Enemy missed
        else if transform.translation.x < -BALL_MAX_X {
            scores.player += 1;

            // Bring back ball to default position once player scores
            // But reduce y speed a bit to let player react
            transform.translation = Vec3::ZERO;
            ball.velocity = Vec2::new(BALL_SPEED_X * ( 1.0 + BALL_SPEED_INCREASE), 0.0);
        }        
    }
}


fn enemy_movement(
    time: Res<Time>,
    ball_query: Query<&Transform, With<Ball>>,
    mut enemy_query: Query<(&Paddle, &mut Transform),
        (With<Enemy>, Without<Ball>)>,
) {
    let Some(ball_transform) = ball_query.iter().next() else {
        return;
    };

    let ball_y = ball_transform.translation.y;

    for (paddle, mut transform) in &mut enemy_query {
        let difference = ball_y - transform.translation.y;

        if difference.abs() > ENEMY_DISTANCE {
            transform.translation.y += difference.signum() * paddle.speed * time.delta_secs();
        }
        
        transform.translation.y = transform.translation.y.clamp(-PADDLE_MAX_Y, PADDLE_MAX_Y);
    }
}

/// For now, this assumes that right paddle is the user paddle 
/// and left paddle is the enemy paddle
fn paddle_collisions(
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    player_query: Query<&Transform,
        (With<Player>, Without<Ball>)>,
    enemy_query: Query<&Transform,
        (With<Enemy>, Without<Ball>)>,
) {
    let (Some(player_transform), Some(enemy_transform)) =
        (player_query.iter().next(), enemy_query.iter().next()) 
    else {
        return;
    };

    let required_x_distance = (BALL_SIZE + PADDLE_WIDTH) / 2.0;
    let required_y_distance = (BALL_SIZE + PADDLE_HEIGHT) / 2.0;

    for (mut ball, mut ball_transform) in &mut ball_query {
        let ball_position = ball_transform.translation;

        // Moving right; where the player paddle is
        if ball.velocity.x > 0.0 
            && (ball_position.x - player_transform.translation.x).abs()
            <= required_x_distance
            && (ball_position.y - player_transform.translation.y).abs()
            <= required_y_distance 
        {
            // println!("Hit right paddle!");
            // Move/stop it right where they touch
            ball_transform.translation.x = player_transform.translation.x - required_x_distance;

            // See how far above/below from paddle's center we have hit
            // If we get positive; we have hit above the center
            // If we get negative, we have hit below the center
            // If we get 0, we have hit right at the center
            let offset = ball_position.y - player_transform.translation.y;

            // We divide offset with half-height of the paddle to normalize
            // the offset to get a relative measurement based on the usable
            // vertical range of the paddle, which is the length from center
            // not the whole length
            // 
            // That's what I studied in class; maybe in movement in inertia
            // chapter; idk
            // 
            // We clamp it between -1 to +1 to account for the situation where
            // the ball can hit paddle while its center is above/below the paddles
            // vertical boundaries, this happens cuz we are using sizes/lengths 
            // of both objects.
            // Clamping between -1 to +1 ensures the ball doesn't bounce/reflect
            // aggresively if it hit at the very edge
            let hit_ratio = 
                (offset / (PADDLE_HEIGHT / 2.0))
                .clamp(-1.0, 1.0);
                
            // What final angle we have got based on the ratio and max angle we have set
            let angle = hit_ratio * MAX_BOUNCE_ANGLE;

            // Take the magnitude of the velocity (speed) which is always positive and 
            // doesn't have any direction
            // 
            // Here, we increase the speed of the ball based on how steeply it touched the paddle
            let previous_speed = ball.velocity.length();

            // This formula increases the speed of the ball by BALL_SPEED_INCREASE * 100% of hit ratio
            // when hit ratio is maximum (1.0) and by 2% when hit ratio is 0.1
            // 
            // Basically x = x * ( 1 + ratio * BALL_SPEED_INCREASE)
            let speed = previous_speed * (1.0 + hit_ratio.abs() * BALL_SPEED_INCREASE);
            let speed = speed.clamp(BALL_SPEED, BALL_MAX_SPEED);

            // .signum() gives us +1 if the velocity x is positive (ball going right)
            // -1 if velocity x is negative (ball going left)
            // 0 if velocity x is zero (when ball is not travelling, ig?)
            // 
            // So we simply reverse whatever direction ball was going in
            // Eg: reversed towards left if the ball was going right
            let direction_x = -ball.velocity.x.signum();

            // Vx = vcos(theta)
            // Here, direction_x * speed is velocity(v),
            // theta is angle
            // so our formula satisfies:
            ball.velocity.x = direction_x * speed * angle.cos();

            // Vy = vsin(theta)
            // We use original speed because we want to keep the speed 
            // of the ball same and just change the direction via sin(angle)
            ball.velocity.y = speed * angle.sin();

            // I hope the formulas are correct cuz that's what I remembered
            // from what I had studied
        } 
        // Moving left; towards enemy paddle
        else if ball.velocity.x < 0.0 
            && (ball_position.x - enemy_transform.translation.x).abs()
            <= required_x_distance
            && (ball_position.y - enemy_transform.translation.y).abs()
            <= required_y_distance
        {
            // println!("Hit left paddle!");
            // Push it required_x_distance right to stop it from overlapping more
            ball_transform.translation.x = enemy_transform.translation.x + required_x_distance;

            let offset = ball_position.y - enemy_transform.translation.y;
    
            let hit_ratio = 
                (offset / (PADDLE_HEIGHT / 2.0))
                .clamp(-1.0, 1.0);
                
            let angle = hit_ratio * MAX_BOUNCE_ANGLE;
    
            // Here, we increase the speed of the ball based on how steeply it touched the paddle
            let previous_speed = ball.velocity.length();
            // This formula increases the speed of the ball by BALL_SPEED_INCREASE * 100% of hit ratio
            // when hit ratio is maximum (1.0) and by 2% when hit ratio is 0.1
            // 
            // Basically x = x * ( 1 + ratio * BALL_SPEED_INCREASE)
            let speed = previous_speed * (1.0 + hit_ratio.abs() * BALL_SPEED_INCREASE);
            let speed = speed.clamp(BALL_SPEED, BALL_MAX_SPEED);

            let direction_x = -ball.velocity.x.signum();
    
            ball.velocity.x = direction_x * speed * angle.cos();
            ball.velocity.y = speed * angle.sin();
        }
    }
}