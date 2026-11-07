
use crate::{Ball, Canvas, Rect, Color};
use crate::utils;

#[derive(Debug, Copy, Clone)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,

    pub side: CanvasSide,
    pub kind: PaddleKind,
}

pub struct Paddles {
    pub user: Paddle,
    pub auto: Paddle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CanvasSide {
    Right,
    Left,
}

#[derive(Debug, Copy, Clone)]
pub enum PaddleKind {
    Player,
    Enemy,
}

#[allow(unused)]
impl Paddle {
    pub const SPEED: f32 = 300.0;
    pub const WIDTH: f32 = 15.0;
    
    pub fn update(
        &mut self,
        dt: f32,
        direction: Option<f32>,
        canvas: &Canvas,
        ball: &Ball
    ) {
        match self.kind {
            PaddleKind::Player => self.update_player(dt, direction, canvas),
            PaddleKind::Enemy => self.update_bot(dt, canvas, ball),
        }
    }

    fn update_player(
        &mut self,
        dt: f32,
        direction: Option<f32>,
        canvas: &Canvas,
    ) {
        if let Some(direc) = direction {
            self.y += dt * direc * self.speed;
        }
        self.y = self.y.clamp(0.0, canvas.height as f32 - self.height);
    }

    fn update_bot(
        &mut self,
        dt: f32,
        canvas: &Canvas,
        ball: &Ball
    ) {
        // println!("Bot hasn't been updated!");
    }

    pub fn hit_ball(&self, ball: &Ball) -> bool {
        utils::rects_overlap(self.to_rect(), ball.to_rect())
    }

    pub fn draw(&self, canvas: &mut Canvas, color: Color) {
        canvas.fill_rect(self.to_rect(), color);
    }

}

#[allow(unused)]
impl Paddle {
    pub fn new(x: f32, canvas: &Canvas, kind: PaddleKind) -> Self {
        Self {
            x,
            y: canvas.height as f32 / 4.5,
            width: Self::WIDTH,
            height: canvas.height as f32 / 4.5,
            speed: Self::SPEED,

            side: CanvasSide::Left,
            kind,
        }
    }

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    pub fn kind(&mut self, kind: PaddleKind) -> &mut Self {
        self.kind = kind;
        self
    }

    fn right(&self, canvas: &Canvas) -> f32 {
        canvas.width as f32 - self.width
    }
    
    pub fn to_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height
        }
    }

    pub fn set_left(&mut self) {
        self.x = 0.0;
        self.side = CanvasSide::Left;
    }

    pub fn is_left(&self) -> bool {
        self.x == 0.0 && self.side == CanvasSide::Left
    }

    pub fn set_right(&mut self, canvas: &mut Canvas) {
        self.x = self.right(canvas);
        self.side = CanvasSide::Right;
    }

    pub fn is_right(&self, canvas: &Canvas) -> bool {
        self.x == self.right(canvas) && self.side == CanvasSide::Right
    }
}
