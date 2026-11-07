
use crate::*;

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            background_color: Color::BACKGROUND,
            bytes: vec![0; width * height * BYTES_PER_PIXEL]
        }
    }

    pub fn to_u32_pixels(&self, buffer: &mut Vec<u32>) {
        buffer.clear();

        for bytes in self.bytes.chunks(BYTES_PER_PIXEL) {
            let red = bytes[0] as u32;
            let green = bytes[1] as u32;
            let blue = bytes[2] as u32;

            let byte = red << 16 | green << 8 | blue;
            buffer.push(byte)
        }
    }
    
    pub fn set_pixel(&mut self, column: usize, row: usize, color: Color) {
        let is_outside_canvas = column >= self.width || row >= self.height;
        if is_outside_canvas { return; }

        let pixel_index = row * self.width + column;
        let first_byte = pixel_index * BYTES_PER_PIXEL;

        self.bytes[first_byte] = color.red;
        self.bytes[first_byte + 1] = color.green;
        self.bytes[first_byte + 2] = color.blue;
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x = rect.x as usize;
        let y = rect.y as usize;

        for row in y..y + rect.height as usize {
            for column in x..x + rect.width as usize {
                self.set_pixel(column, row, color);
            }
        }
    }

    fn draw_border(&mut self, rect: Rect, border: Border) {        
        let thickness = border.thickness;
        let border_color = border.color;
        
        let mut vertical = Rect {
            x: (rect.x - thickness).clamp(0.0, self.width as f32 - thickness),
            y: (rect.y - thickness).clamp(0.0, self.height as f32 - thickness),
            width: thickness,
            height: rect.height + 2.0 * thickness,
        };
        self.fill_rect(vertical, border_color);

        vertical.x = rect.x + rect.width;
        self.fill_rect(vertical, border_color);
        
        let mut horizontal = Rect {
            x: rect.x,
            y: (rect.y - thickness).clamp(0.0, self.height as f32 - thickness),
            width: rect.width,
            height: thickness
        };
        self.fill_rect(horizontal, border_color);

        horizontal.y = rect.y + rect.height;
        self.fill_rect(horizontal, border_color);
    }
    
    pub fn clear(&mut self) {
        self.fill_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                width: self.width as f32,
                height: self.height as f32
            },
            self.background_color
        );
    }
}

impl Rect {
    pub fn contains(&self, point: Point) -> bool {
        utils::ranges_overlap(
            point.x, self.x + self.width,
            point.y, self.y + self.height
        )
    }
}

#[allow(unused)]
impl Button {
    pub const DEFAULT_COLOR: Color = Color::ORANGE;

    pub fn new(rect: Rect, color: Color, border: Border) -> Button {
        Button {
            rect,
            color,
            border,
            state: ButtonState::Normal,
        }
    }
    
    pub fn draw(&mut self, canvas: &mut Canvas) {
        let color = match self.state {
            ButtonState::Normal => self.color,
            ButtonState::Hovered => Color::GREEN,
            ButtonState::Pressing => Color::BLUE
        };
        
        canvas.fill_rect(self.rect, color);
        canvas.draw_border(self.rect, self.border);
    }
    
    pub fn with_border(&self) -> Rect {
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

    pub fn contains(&self, point: Point) -> bool {
        self.with_border().contains(point)
    }

    #[allow(unused)]
    pub fn contains_with_no_border(&self, point: Point) -> bool {
        self.rect.contains(point)
    }
    
    pub fn get_state(contains: bool, left_is_down: bool, was_down: bool) -> ButtonState {
        match (contains, left_is_down, was_down) {
            (true, true, _) => ButtonState::Pressing,
            (true, false, false) => ButtonState::Hovered,
            
            _ => ButtonState::Normal
        }
    }
}
