
use std::time::Instant;

use minifb::{Window, WindowOptions};
// use minifb::{MouseButton, MouseMode};
use minifb::{Key, KeyRepeat};

mod utils;
mod impls;
mod paddle;
mod ball;

use paddle::*;
use ball::*;

const NAME: &str = "UPong";

const BYTES_PER_PIXEL: usize = 3;
// const MAX_COLOR_VALUE: usize = 255;

const FPS: usize = 60;


const CANVAS_WIDTH: usize = 720;
const CANVAS_HEIGHT: usize = 480;

#[derive(Debug, Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(unused)]
impl Color {
    const RED: Color = Color { red: 255, green: 0, blue: 0 };
    const LIGHT_RED: Color = Color { red: 255, green: 127, blue: 127 };
    
    const GREEN: Color = Color { red: 0, green: 255, blue: 0 };
    const LIGHT_GREEN: Color = Color { red: 144, green: 238, blue: 144 };
    
    const BLUE: Color = Color { red: 0, green: 0, blue: 255 };
    const LIGHT_BLUE: Color = Color { red: 173, green: 216, blue: 230 };
    
    const ORANGE: Color = Color { red: 255, green: 128, blue: 0 };
    const LIGHT_ORANGE: Color = Color { red: 255, green: 180, blue: 80 };

    const WHITE: Color = Color { red: 255, green: 255, blue: 255 };

    const BLACK: Color = Color { red: 0, green: 0, blue: 0 };
    
    const DARK_GREY: Color = Color { red: 32, green: 32, blue: 32 };
    const BACKGROUND: Color = Color::BLACK;
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

#[derive(Copy, Clone)]
struct Button {
    rect: Rect,
    color: Color,
    border: Border,
    state: ButtonState,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
enum ButtonState {
    Normal,
    Hovered,
    Pressing,
}

#[derive(Debug, Clone)]
struct Canvas {
    width: usize,
    height: usize,
    background_color: Color,
    bytes: Vec<u8>,
}

#[derive(Copy, Clone)]
struct Border {
    thickness: f32,
    color: Color,
}

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    canvas.clear();

    // The default values for paddle are already set;
    // but we can change them via method chaining on this paddle
    // if we want
    let mut paddle = Paddle::new(0.0, &canvas, PaddleKind::Player);
    paddle.set_left();

    // The paddle that will play on its own
    // I have set x same for both player and enemy even though that
    // will cause an overlap because .set_right() will automatically
    // set it to the far right, which will not cause an overlap
    // because player paddle is on the left
    let mut automatic_paddle = Paddle::new(0.0, &canvas, PaddleKind::Enemy);
    automatic_paddle.set_right(&mut canvas);

    let mut ball = Ball::new(None, None, &canvas);
    
    let mut window = Window::new(
        NAME,
        canvas.width,
        canvas.height,
        WindowOptions::default()
    ).unwrap();

    window.set_target_fps(FPS);

    let mut buffer: Vec<u32> = Vec::new();

    let mut last_frame = Instant::now();

    let mut has_started = false;
    
    while window.is_open() {     
        canvas.clear(); 

        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        if has_started {
            ball.update(dt, &canvas, &Paddles {
                user: paddle,
                auto: automatic_paddle
            });
            
            automatic_paddle.update(dt, None, &canvas, &ball);
        }

        let mut paddle_direction = 0.0;
        
        if window.is_key_down(Key::Up) {paddle_direction -= 1.0};
        if window.is_key_down(Key::Down) {paddle_direction += 1.0};

        ball.draw(&mut canvas, Color::WHITE);
        automatic_paddle.draw(&mut canvas, Color::WHITE);
        
        paddle.update(dt, Some(paddle_direction), &canvas, &ball);
        paddle.draw(&mut canvas, Color::WHITE);

        if ball.did_player_win(&canvas, &automatic_paddle) {
            println!("You won!");
            return;
        } else if ball.did_player_lose(&canvas, &paddle) {
            eprintln!("You lost!");
            return;
        }
        
        canvas.to_u32_pixels(&mut buffer);
        window.update_with_buffer(&buffer, canvas.width, canvas.height).unwrap();

        if !has_started && window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            has_started = true;
            println!("Game has started!")
        }
    }
}
