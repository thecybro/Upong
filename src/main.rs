
use std::time::Instant;

use minifb::{Window, WindowOptions};
// use minifb::{MouseButton, MouseMode};
use minifb::{Key, KeyRepeat};

mod utils;
mod impls;

const NAME: &str = "UPong";

const BYTES_PER_PIXEL: usize = 3;
// const MAX_COLOR_VALUE: usize = 255;

const FPS: usize = 60;

const PADDLE_SPEED: f32 = 300.0;

const BALL_SIDE: f32 = 15.0; // single ball side just means width = height aka square
const BALL_SPEED_X: f32 = 300.0;
const BALL_SPEED_Y: f32 = 300.0;

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

#[derive(Debug)]
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32, // PPS (pixels per sec)
}

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    side: f32, // single side just means width & height are same
    velocity_x: f32, // PPS (pixels per sec)
    velocity_y: f32
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

    let mut paddle = Paddle { 
        x: 0.0,
        y: (canvas.height / 2) as f32,
        width: 15.0,
        height: canvas.height as f32/ 4.5,
        speed: PADDLE_SPEED
    };

    let mut ball = Ball {
        x: canvas.width as f32 / 2.0,
        y: canvas.height as f32 / 2.0,
        side: BALL_SIDE,
        velocity_x: BALL_SPEED_X,
        velocity_y: BALL_SPEED_Y
    };
    
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
            ball.update(dt, &canvas, &paddle);
            ball.draw(&mut canvas, Color::WHITE);
        }

        let mut paddle_direction = 0.0;
        
        if window.is_key_down(Key::Up) {paddle_direction -= 1.0};
        if window.is_key_down(Key::Down) {paddle_direction += 1.0};
        
        paddle.update(dt, paddle_direction, &canvas, &ball);
        paddle.draw(&mut canvas, Color::WHITE);
        
        canvas.to_u32_pixels(&mut buffer);
        window.update_with_buffer(&buffer, canvas.width, canvas.height).unwrap();

        if !has_started && window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            has_started = true;
        }
    }
}
