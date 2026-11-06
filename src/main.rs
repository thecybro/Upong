
use std::fs::File;
use std::io::Write;

use minifb::{Window, WindowOptions};
use minifb::{MouseButton, MouseMode};

const BYTES_PER_PIXEL: usize = 3;
const MAX_COLOR_VALUE: usize = 255;
const FPS: usize = 60;

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
    
    const DARK_GREY: Color = Color { red: 32, green: 32, blue: 32 };
    const BACKGROUND: Color = Color::DARK_GREY;
}

#[derive(Copy, Clone)]
struct Rect {
    left: usize,
    top: usize,
    width: usize,
    height: usize
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
    thickness: usize,
    color: Color,
}

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

        let left = rect.left.saturating_sub(t);
        let top = rect.top.saturating_sub(t);

        let right = rect.left + rect.width + t;
        let bottom = rect.top + rect.height + t;
        
        Rect {
            left,
            top,
            width: right - left,
            height: bottom - top,
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
        // We convert values to f64 so we can safely compare
        // f32 x/y points to our usize coordinates without
        // losing much/any precision
        let x = point.x as f64;
        let y = point.y as f64;
        
        x >= self.left as f64 &&
        x < (self.left + self.width) as f64 &&
        
        y >= self.top as f64 &&
        y < (self.top + self.height) as f64
    }
}

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
        for row in rect.top..rect.top + rect.height {
            for column in rect.left..rect.left + rect.width {
                self.set_pixel(column, row, color);
            }
        }
    }

    fn draw_border(&mut self, rect: Rect, border: Border) {
        let mut vertical = Rect {
            left: rect.left.saturating_sub(border.thickness),
            top: rect.top.saturating_sub(border.thickness),
            width: border.thickness,
            height: rect.height + 2 * border.thickness,
        };
        self.fill_rect(vertical, border.color);

        vertical.left = rect.left + rect.width;
        self.fill_rect(vertical, border.color);
        
        let mut horizontal = Rect {
            left: rect.left,
            top: rect.top.saturating_sub(border.thickness),
            width: rect.width,
            height: border.thickness
        };
        self.fill_rect(horizontal, border.color);

        horizontal.top = rect.top + rect.height;
        self.fill_rect(horizontal, border.color);
    }
    
    fn clear(&mut self) {
        let canvas = Rect {
            left: 0,
            top: 0,
            width: self.width,
            height: self.height 
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
    let mut canvas = Canvas::new(200, 120);
    canvas.clear();

    // Canvas width: rect left + rect width
    // Canvas height: rect top + rect height
    let rect = Rect { left: 20, top: 20, width: 160, height: 80 };    
    let border = Border { thickness: 5, color: Color::RED };
    
    let mut button = Button::new(rect, Button::DEFAULT_COLOR, border);
    
    let mut window = Window::new(
        "Ugame",
        canvas.width,
        canvas.height,
        WindowOptions::default()
    ).unwrap();

    window.set_target_fps(FPS);

    let mut buffer: Vec<u32> = Vec::new();
    let mut was_down: bool = false;
    
    while window.is_open() {       
        canvas.clear(); 
        // Draw into canvas here...

        let left_is_down = window.get_mouse_down(MouseButton::Left);
        
        let contains = if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            // let column = mouse_x as usize;
            // let row = mouse_y as usize;

            button.contains(&Point {
                x: mouse_x,
                y: mouse_y
            })
            
            // println!("Mouse is at column {column} and row {row}")
        } else {
            // println!("Mouse is outside the window")
            false
        };

        if contains && !left_is_down && was_down {
            println!("You just clicked!");
        }
        
        button.state = Button::get_state(contains, left_is_down, was_down);
        button.draw(&mut canvas);
        was_down = left_is_down;
        
        canvas.to_u32_pixels(&mut buffer);
        window.update_with_buffer(&buffer, canvas.width, canvas.height).unwrap();
    }
}
