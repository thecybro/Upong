
use bevy::prelude::*;

use crate::*;

/// For now, this assumes that right paddle is the user paddle
/// and left paddle is the enemy paddle
pub(crate) fn paddle_collisions(
    difficulty: Res<SelectedDifficulty>,
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    player_query: Query<&Transform, (With<Player>, Without<Ball>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Ball>)>,
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
            && (ball_position.x - player_transform.translation.x).abs() <= required_x_distance
            && (ball_position.y - player_transform.translation.y).abs() <= required_y_distance
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
            let hit_ratio = (offset / (PADDLE_HEIGHT / 2.0)).clamp(-1.0, 1.0);

            // What final angle we have got based on the ratio and max angle we have set
            let angle = hit_ratio * MAX_BOUNCE_ANGLE;

            // Take the magnitude of the velocity (speed) which is always positive and
            // doesn't have any direction
            //
            // Here, we increase the speed of the ball based on how steeply it touched the paddle
            let previous_speed = ball.velocity.length();

            // This formula scales the ball speed according to the hit ratio.
            let speed = previous_speed * (1.0 + hit_ratio.abs() * difficulty.ball_speed_increase());
            let speed = speed.clamp(difficulty.ball_speed(), difficulty.ball_max_speed());

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
            && (ball_position.x - enemy_transform.translation.x).abs() <= required_x_distance
            && (ball_position.y - enemy_transform.translation.y).abs() <= required_y_distance
        {
            // println!("Hit left paddle!");
            // Push it required_x_distance right to stop it from overlapping more
            ball_transform.translation.x = enemy_transform.translation.x + required_x_distance;

            let offset = ball_position.y - enemy_transform.translation.y;

            let hit_ratio = (offset / (PADDLE_HEIGHT / 2.0)).clamp(-1.0, 1.0);

            let angle = hit_ratio * MAX_BOUNCE_ANGLE;

            // Here, we increase the speed of the ball based on how steeply it touched the paddle
            let previous_speed = ball.velocity.length();
            // Scale the ball speed according to the hit ratio.
            let speed = previous_speed * (1.0 + hit_ratio.abs() * difficulty.ball_speed_increase());
            let speed = speed.clamp(difficulty.ball_speed(), difficulty.ball_max_speed());

            let direction_x = -ball.velocity.x.signum();

            ball.velocity.x = direction_x * speed * angle.cos();
            ball.velocity.y = speed * angle.sin();
        }
    }
}
