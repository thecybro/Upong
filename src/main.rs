use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 720.0;
const WINDOW_HEIGHT: f32 = 480.0; 

const BALL_SIZE: f32 = 15.0;
const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = WINDOW_HEIGHT / 4.5;

const ENEMY_DISTANCE: f32 = 10.0; // 10 pixels

const REFRESH_RATE: f64 = 60.0;

#[derive(Component)] struct Ball { velocity: Vec2 }
#[derive(Component)] struct Paddle { speed: f32 }
#[derive(Component)] struct Player;
#[derive(Component)] struct Enemy;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            title: "Upong".to_string(),
                            resolution: (720, 480).into(),
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
        .add_systems(Startup, setup)
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
            .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Ball (at center)
    commands.spawn((
        Ball {
            velocity: Vec2::new(300.0, -300.0)
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::splat(BALL_SIZE),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    // Player paddle (at right)
    commands.spawn((
        Player,
        Paddle {
            speed: 300.0,
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
        ),
        Transform::from_xyz(
            WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0,
            0.0,
            0.0
        )
    ));

    // Enemy paddle (at left)
    commands.spawn((
        Enemy,
        Paddle {
            speed: 240.0,
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        ),
        Transform::from_xyz(
            - WINDOW_WIDTH / 2.0 + PADDLE_WIDTH / 2.0,
            0.0,
            0.0
        )
    ));
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

        let max_y = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;

        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
    }
}


fn ball_movement(
    mut query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    time: Res<Time>,
) {
    for (mut ball, mut transform) in &mut query {
        let dt = time.delta_secs();

        transform.translation += ball.velocity.extend(0.0) * dt;

        let max_x = WINDOW_WIDTH / 2.0 - BALL_SIZE / 2.0;
        let max_y = WINDOW_HEIGHT / 2.0 - BALL_SIZE / 2.0;

        // Restrict the ball from going above/below the screen
        if transform.translation.y > max_y {
            transform.translation.y = max_y;
            ball.velocity.y = -ball.velocity.y;
        } else if transform.translation.y < -max_y {
            transform.translation.y = -max_y;
            ball.velocity.y = -ball.velocity.y;
        }

        // FIXME: Remove this logic once we implement ball-paddle collision logic
        // and register ball touching max of either x axes as score to the
        // respective entity (eg: player/enemy)
        if transform.translation.x > max_x {
            transform.translation.x = max_x;
            ball.velocity.x = -ball.velocity.x;
        } else if transform.translation.x < -max_x {
            transform.translation.x = -max_x;
            ball.velocity.x = -ball.velocity.x
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
        
        let max_y = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;

        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
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

            let offset = ball_position.y - player_transform.translation.y;
    
            let hit_ratio = (
                    offset / (PADDLE_HEIGHT / 2.0)
                )
            .clamp(-1.0, 1.0);
    
            let max_angle = 60.0_f32.to_radians();
            let angle = hit_ratio * max_angle;
    
            let speed = ball.velocity.length();
    
            let direction_x = -ball.velocity.x.signum();
    
            ball.velocity.x = direction_x * speed * angle.cos();
            ball.velocity.y = speed * angle.sin();
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
    
            let hit_ratio = (
                    offset / (PADDLE_HEIGHT / 2.0)
                )
            .clamp(-1.0, 1.0);
    
            let max_angle = 60.0_f32.to_radians();
            let angle = hit_ratio * max_angle;
    
            let speed = ball.velocity.length();
    
            let direction_x = -ball.velocity.x.signum();
    
            ball.velocity.x = direction_x * speed * angle.cos();
            ball.velocity.y = speed * angle.sin();
        }
    }
}