use bevy::prelude::*;

use crate::*;

pub(crate) fn player_movement(
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

pub(crate) fn ball_movement(
    time: Res<Time>,
    difficulty: Res<SelectedDifficulty>,
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
            ball.velocity.y = -(ball.velocity.y / difficulty.ball_speed_decrease());
            ball.velocity.y = ball
                .velocity
                .y
                .clamp(-difficulty.ball_speed_y(), difficulty.ball_max_speed_y());
        } else if transform.translation.y < -BALL_MAX_Y {
            transform.translation.y = -BALL_MAX_Y;
            ball.velocity.y = -(ball.velocity.y / difficulty.ball_speed_decrease());
            ball.velocity.y = ball
                .velocity
                .y
                .clamp(difficulty.ball_speed_y(), difficulty.ball_max_speed_y());
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
            ball.velocity = Vec2::new(
                difficulty.ball_speed_x() * (1.0 + difficulty.ball_speed_increase()),
                0.0,
            );
        }
        // When ball touches left edge (where enemy is)
        // Enemy missed
        else if transform.translation.x < -BALL_MAX_X {
            scores.player += 1;

            // Bring back ball to default position once player scores
            // But reduce y speed a bit to let player react
            transform.translation = Vec3::ZERO;
            ball.velocity = Vec2::new(
                difficulty.ball_speed_x() * (1.0 + difficulty.ball_speed_increase()),
                0.0,
            );
        }
    }
}

pub(crate) fn enemy_movement(
    time: Res<Time>,
    difficulty: Res<SelectedDifficulty>,
    ball_query: Query<&Transform, With<Ball>>,
    mut enemy_query: Query<(&Paddle, &mut Transform), (With<Enemy>, Without<Ball>)>,
) {
    let Some(ball_transform) = ball_query.iter().next() else {
        return;
    };

    let ball_y = ball_transform.translation.y;

    for (paddle, mut transform) in &mut enemy_query {
        let difference = ball_y - transform.translation.y;

        if difference.abs() > difficulty.enemy_distance() {
            transform.translation.y += difference.signum() * paddle.speed * time.delta_secs();
        }

        transform.translation.y = transform.translation.y.clamp(-PADDLE_MAX_Y, PADDLE_MAX_Y);
    }
}
