
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

impl Color {
    const RED: Color = Color { red: 255, green: 0, blue: 0 };
    const ORANGE: Color = Color { red: 255, green: 128, blue: 0 };
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct Canvas {
    width: usize,
    height: usize,
    background: Color,
    bytes: Vec<u8>,
}

#[derive(Copy, Clone)]
struct Border {
    thickness: usize,
    color: Color,
}

impl Button {
    fn new(rect: Rect, color: Color, border: Border) -> Button {
        Button {
            rect,
            color,
            border,
        }
    }
    
    fn draw(&self, canvas: &mut Canvas) {
        canvas.fill_rect(self.rect, self.color);
        canvas.draw_border(self.rect, self.border);
    }

    fn contains(&self, point: &Point) -> bool {
        let rect = self.rect;
        let border_thickness = self.border.thickness;
        
        let left = rect.left;
        let top = rect.top - border_thickness;

        let x = point.x as usize;
        let y = point.y as usize;
        
        x >= left &&
        x < left + rect.width  &&
        y >= top &&
        y < top + rect.height
    }
    
    #[allow(unused)]
    fn clear(&mut self, canvas: &mut Canvas) {
        self.color = canvas.background;
        self.border.color = canvas.background;
        self.draw(canvas);
    }
}

impl Rect {
    #[allow(unused)]
    fn contains(&self, point: (usize, usize)) -> bool {
        point.0 >= self.left &&
        point.0 < self.left + self.width &&
        point.1 >= self.top &&
        point.1 < self.top + self.height
    }
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height, background: Color::BACKGROUND, bytes: vec![0; width * height * BYTES_PER_PIXEL] }
    }

    fn to_u32_pixels(&self) -> Vec<u32> {
        let mut pixels: Vec<u32> = Vec::new();
        for bytes in self.bytes.chunks(BYTES_PER_PIXEL) {
            let red = bytes[0] as u32;
            let green = bytes[1] as u32;
            let blue = bytes[2] as u32;

            let byte = red << 16 | green << 8 | blue;
            pixels.push(byte)
            
        }
        pixels
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
        self.fill_rect(canvas, self.background)
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

    // let pixels = canvas.to_u32_pixels()[0];
    // println!("{pixels:#?}");
    
    // Canvas width: rect left + rect width
    // Canvas height: rect top + rect height
    let rect = Rect { left: 20, top: 20, width: 160, height: 80 };    
    let border = Border { thickness: 5, color: Color::RED };
    let button = Button::new(rect, Color::ORANGE, border);
    button.draw(&mut canvas);
    
    // canvas.save("out.ppm");

    // button.clear(&mut canvas);

    // canvas.save("out2.ppm");

    let mut window = Window::new(
        "Ugame",
        canvas.width,
        canvas.height,
        WindowOptions::default()
    ).unwrap();

    window.set_target_fps(FPS);

    while window.is_open() {        
        // Draw into canvas here...

        let left_is_down = window.get_mouse_down(MouseButton::Left);

        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            let column = mouse_x as usize;
            let row = mouse_y as usize;

            let point = Point {
                x: mouse_x,
                y: mouse_y
            };
            
            if button.contains(&point) {
                if left_is_down {
                    println!("You clicked the button!")
                }
            }
            
            println!("Mouse is at column {column} and row {row}")
        } else {
            println!("Mouse is outside the window")
        }
        
        let pixels = canvas.to_u32_pixels();
        window.update_with_buffer(&pixels, canvas.width, canvas.height).unwrap();
    }
}
