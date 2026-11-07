
use std::fs::File;
use std::io::Write;

use std::time::Instant;

use minifb::{Window, WindowOptions};
// use minifb::{MouseButton, MouseMode};
use minifb::{Key, KeyRepeat};

const NAME: &str = "UPong";

const BYTES_PER_PIXEL: usize = 3;
const MAX_COLOR_VALUE: usize = 255;

const FPS: usize = 60;

const PADDLE_SPEED: f32 = 300.0;

const BALL_SIDE: f32 = 15.0; // single ball side just means width = height aka square
const BALL_SPEED_X: f32 = 200.0;
const BALL_SPEED_Y: f32 = 200.0;

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

#[derive(Copy, Clone)]
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

fn bigger(value1: f32, value2: f32) -> f32 {
    value1.max(value2)
}

fn smaller(value1: f32, value2: f32) -> f32 {
    value1.min(value2)
}

fn in_range(value: f32, min: f32, max: f32) -> bool {
    /* This will break if max is less than min;
       So we check based on which one is less/more
       regardless of what was passed here
    */
    // (value >= min) && (value <= max)
    
    (value >= smaller(min, max)) && (value <= bigger(min, max))
}

fn ranges_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    bigger(min1, max1) >= smaller(min2, max2)
    && 
    smaller(min1, max1) <= bigger(min2, max2)
}

#[allow(unused)]
impl Button {
    const DEFAULT_COLOR: Color = Color::ORANGE;

    fn new(rect: Rect, color: Color, border: Border) -> Button {
        Button {
            rect,
            color,
            border,
            state: ButtonState::Normal,
        }
    }
    
    fn draw(&self, canvas: &mut Canvas) {
        // We take the current color of the button
        // and draw with it because no method changes
        // that color once the button is initialized
        // with one.
        // So we always have a default color to use
        // without hardcoding a default color here when 
        // the state is normal
        // let mut color = self.color;
        let color = match self.state {
            // We dont have a pressed state here because that's
            // not meant to be a mouse state; it's an action driver
            // that does something the moment it is registered, we
            // dont store/compare it here; it happens at the detection
            // site
            ButtonState::Normal => {

                self.color
            },
            ButtonState::Hovered => {

                Color::GREEN
            },
            ButtonState::Pressing => {
                
                Color::BLUE
            }
        };
        canvas.fill_rect(self.rect, color);
        canvas.draw_border(self.rect, self.border);
    }

    fn with_border(&self) -> Rect {
        let rect = self.rect;
        let t = self.border.thickness;

        let x = (rect.x - t).max(0.0);
        let y = (rect.y - t).max(0.0);

        let right = rect.x + rect.width + t;
        let bottom = rect.y + rect.height + t;
        
        Rect {
            x,
            y,
            width: right - x,
            height: bottom - y,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self.with_border().contains(point)
    }

    #[allow(unused)]
    fn contains_with_no_border(&self, point: &Point) -> bool {
        self.rect.contains(point)
    }
    
    fn get_state(contains: bool, left_is_down: bool, was_down: bool) -> ButtonState {
        match (contains, left_is_down, was_down) {
            (true, true, _) => ButtonState::Pressing,
            (true, false, false) => ButtonState::Hovered,
            
            _ => ButtonState::Normal
        }
    }
}

impl Rect {
    fn contains(&self, point: &Point) -> bool {        
        in_range(point.x, self.x, self.x + self.width)
        &&
        in_range(point.y, self.y, self.y + self.height)
    }
}

impl Paddle {
    fn to_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    fn update(&mut self, dt: f32, direction: f32) {
        self.y += direction * self.speed * dt;
        self.y = self.y.clamp(0.0, CANVAS_HEIGHT as f32 - self.height);
    }

    fn hit_ball(&self, ball: &Ball) -> bool {
        ranges_overlap(
            ball.x, ball.x + ball.side,
            self.x, self.x + self.width
        )
        &&
        ranges_overlap(
            ball.y, ball.y + ball.side,
            self.y, self.y + self.height
        )
    }
    
    fn draw(&self, canvas: &mut Canvas, color: Color) {
        canvas.fill_rect(
            Self::to_rect(&self),
            color
        );
    }
}

impl Ball {
    fn to_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.side,
            height: self.side,
        }
    }

    fn update(&mut self, dt: f32, paddle: &Paddle) {
        self.x += dt * self.velocity_x;        
        self.y += dt * self.velocity_y;

        let bottom_limit = CANVAS_HEIGHT as f32 - self.side;
        let right_limit = CANVAS_WIDTH as f32 - self.side;
        
        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity_y = -self.velocity_y
        }

        if self.x < 0.0 {
            self.x = 0.0;
            self.velocity_x = -self.velocity_x;
        }

        if self.hit_paddle(paddle) {
            println!("Hit the paddle!");

            // println!("Ball: {self:#?}\nPaddle: {paddle:#?}");
            
            self.x = paddle.x + paddle.width;
            self.velocity_x = self.velocity_x.abs();
        }
        
        if self.y > bottom_limit {
            // println!("Hit the bottom!");
            self.y = bottom_limit;
            self.velocity_y = -self.velocity_y;
        }

        if self.x > right_limit {
            // println!("Hit the right!");
            self.x = right_limit;
            self.velocity_x = -self.velocity_x;
        }
    }

    fn hit_paddle(&self, paddle: &Paddle) -> bool {
        ranges_overlap(
            self.x, self.x + self.side,
            paddle.x, paddle.x + paddle.width
        )
        &&
        ranges_overlap(
            self.y, self.y + self.side,
            paddle.y, paddle.y + paddle.height
        )
    }
    
    fn draw(&self, canvas: &mut Canvas, color: Color) {
        canvas.fill_rect(
            Self::to_rect(&self),
            color
        );
    }
}

#[allow(unused)]
impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self { 
            width,
            height,
            background_color: Color::BACKGROUND, 
            bytes: vec![0; width * height * BYTES_PER_PIXEL] 
        }
    }

    // It uses an existing buffer instead of allocating a new
    // Vec everytime we need u32 byte pairs to update the screen
    fn to_u32_pixels(&self, buffer: &mut Vec<u32>) {
        buffer.clear();
        for bytes in self.bytes.chunks(BYTES_PER_PIXEL) {
            let red = bytes[0] as u32;
            let green = bytes[1] as u32;
            let blue = bytes[2] as u32;

            let byte = red << 16 | green << 8 | blue;
            buffer.push(byte)
            
        }
    }
    
    fn set_pixel(&mut self, column: usize, row: usize, color: Color) {
        let is_outside_canvas = column >= self.width || row >= self.height;
        if is_outside_canvas {return;}

        let pixel_index = row * self.width + column;
        let first_byte = pixel_index * BYTES_PER_PIXEL;

        self.bytes[first_byte] = color.red;
        self.bytes[first_byte + 1] = color.green;
        self.bytes[first_byte + 2] = color.blue;
    }

    fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x = rect.x as usize;
        let y = rect.y as usize;

        let height = rect.height as usize;
        let width = rect.width as usize;
        
        for row in y..y + height {
            for column in x..x + width {
                self.set_pixel(column, row, color);
            }
        }
    }

    fn draw_border(&mut self, rect: Rect, border: Border) {
        let thickness = border.thickness;
        
        let mut vertical = Rect {
            x: (rect.x - thickness).clamp(0.0, CANVAS_WIDTH as f32 - thickness),
            y: (rect.y - thickness).clamp(0.0, CANVAS_HEIGHT as f32 - thickness ),
            width: thickness,
            height: rect.height + 2.0 * thickness,
        };
        self.fill_rect(vertical, border.color);

        vertical.x = rect.x + rect.width;
        self.fill_rect(vertical, border.color);
        
        let mut horizontal = Rect {
            x: rect.x,
            y: (rect.y - thickness).clamp(0.0, CANVAS_HEIGHT as f32 - thickness),
            width: rect.width,
            height: border.thickness
        };
        self.fill_rect(horizontal, border.color);

        horizontal.y = rect.y + rect.height;
        self.fill_rect(horizontal, border.color);
    }
    
    fn clear(&mut self) {
        let canvas = Rect {
            x: 0.0,
            y: 0.0,
            width: self.width as f32,
            height: self.height as f32
        };
        self.fill_rect(canvas, self.background_color)
    }

    #[allow(unused)]
    fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();

        write!(file, "P6\n{} {}\n{}\n", self.width, self.height, MAX_COLOR_VALUE).unwrap();

        file.write_all(&self.bytes).unwrap();
    }
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
            ball.update(dt, &paddle);
            ball.draw(&mut canvas, Color::WHITE);
        }

        let mut paddle_direction = 0.0;
        
        if window.is_key_down(Key::Up) {paddle_direction -= 1.0};
        if window.is_key_down(Key::Down) {paddle_direction += 1.0};

        if paddle.hit_ball(&ball) {
            println!("Paddle hit the ball!")
        }
        
        paddle.update(dt, paddle_direction);
        paddle.draw(&mut canvas, Color::WHITE);
        
        canvas.to_u32_pixels(&mut buffer);
        window.update_with_buffer(&buffer, canvas.width, canvas.height).unwrap();

        if !has_started && window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            has_started = true;
        }
    }
}
