
use crate::*;

#[derive(Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub side: f32, // single side just means width & height are same
    pub velocity_x: f32, // PPS (pixels per sec)
    pub velocity_y: f32
}


#[allow(unused)]
impl Ball {
    const SIDE: f32 = 15.0; // single ball side just means width = height aka square
    const SPEED_X: f32 = 300.0;
    const SPEED_Y: f32 = 300.0;

    /// You can pass either (None, None) and it will automatically
    /// put the default values or you can pass actual values without having
    /// to wrap them in Some() and it will use that
    pub fn new<P> (x: P, y: P, canvas: &Canvas) -> Ball 
        where 
            P: Into<Option<f32>>,
    {
        let (ball_x, ball_y) = if let (Some(_x), Some(_y)) = (x.into(), y.into()) {
            (_x, _y)
        } else {
            (canvas.width as f32 / 2.0, canvas.height as f32 / 2.0)
         };
        
        Ball {
            x: ball_x,
            y: ball_y,
            side: Self::SIDE,
            velocity_x: Self::SPEED_X,
            velocity_y: Self::SPEED_Y
        }
    }

    pub fn side(&mut self, side: f32) -> &mut Self {
        self.side = side;
        self
    }

    pub fn velocity_x(&mut self, velocity_x: f32) -> &mut Self {
        self.velocity_x = velocity_x;
        self
    }

    pub fn velocity_y(&mut self, velocity_y: f32) -> &mut Self {
        self.velocity_y = velocity_y;
        self
    }
    
    pub fn to_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            height: self.side,
            width: self.side,
        }
    }

    pub fn hit_paddle(&self, paddle: &Paddle) -> bool {
        utils::rects_overlap(self.to_rect(), paddle.to_rect())
    }
    
    pub fn update(&mut self, dt: f32, canvas: &Canvas, paddles: &Paddles) {
        let user_paddle = paddles.user;
        let auto_paddle = paddles.auto;
        
        self.x += dt * self.velocity_x;
        self.y += dt * self.velocity_y;

        // Remove the check that was reflecting the ball when it hit
        // the left side; with this, we can detect if user/bot has missed
        // the ball and count it as game over
        // if self.x < 0.0 {
        //     self.x = 0.0;
        //     self.velocity_x = -self.velocity_x
        // }

        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity_y = -self.velocity_y;
        }

        // Collision detection on the paddles
        match (self.hit_paddle(&user_paddle), self.hit_paddle(&auto_paddle)) {
            (true, false) => {
                // println!("Hit the user paddle!");

                if user_paddle.is_right(canvas) {
                    self.x = user_paddle.x - self.side;
                } else {
                    self.x = user_paddle.x + user_paddle.width;
                }

                self.velocity_x = -self.velocity_x;
            },
            (false, true) => {
                // println!("Hit the auto paddle!");

                if auto_paddle.is_right(canvas) {
                    self.x = auto_paddle.x - self.side
                } else {
                    self.x = auto_paddle.x + auto_paddle.width
                }

                self.velocity_x = -self.velocity_x;
            },
            (true, true) => {
                eprintln!("Warning: Ball collided with both paddles!");
            },
            _ => {}
        };

        let bottom_limit = canvas.height as f32 - self.side;
        // let side_limit = canvas.width as f32 - self.side;

        // Remove the check that was bouncing the ball when it hit right side,
        // so we can detect it as game over, if it happened
        // if self.x > side_limit {
        //     // println!("Side limit reached, stop and flip!");
        //     self.x = side_limit;
        //     self.velocity_x = -self.velocity_x
        // }

        if self.y > bottom_limit {
            // println!("Bottom limit reached, stop and flip!");
            self.y = bottom_limit;
            self.velocity_y = -self.velocity_y;
        }
    }

    fn touched_right_wall(&self, canvas: &Canvas) -> bool {
        self.x + self.side >= canvas.width as f32
    }

    fn touched_left_wall(&self, canvas: &Canvas) -> bool {
        self.x <= 0.0
    }
        
    /// Gave over if the ball has surpassed either side of the canvas
    /// 
    /// We use this helper to determine whether or not the ball has actually
    /// touched either side; before we try to decide who has lost/won
    fn is_game_over(&self, canvas: &Canvas) -> bool {
        self.touched_right_wall(canvas)
        || self.touched_left_wall(canvas)
    }

    /// Take the user's paddle and see if it is at the right;
    /// if it is; see if the ball has touched right wall, if we
    /// found that it has, player has missed to touch the ball,
    /// so they lost
    /// 
    /// If user's paddle is at left, we see if the ball has touched
    /// left wall, and player has lost if it has
    /// 
    /// If the ball has not touched the wall of the player's side,
    /// player is not determined to have lost
    pub fn did_player_lose(&self, canvas: &Canvas, user_paddle: &Paddle) -> bool {
        if user_paddle.kind != PaddleKind::Player
        || !self.is_game_over(canvas) 
            {return false}

        if user_paddle.is_right(canvas) {
            return self.touched_right_wall(canvas)
        }

        self.touched_left_wall(canvas)
    }

    /// Player wins only if enemy loses,
    /// so we take enemy's paddle and see if the ball has touched the side
    /// enemy is in; 
    /// if it has, player has won
    pub fn did_player_win(&self, canvas: &Canvas, enemy_paddle: &Paddle) -> bool {
        if enemy_paddle.kind != PaddleKind::Enemy
        || !self.is_game_over(canvas) 
            {return false}

        if enemy_paddle.is_right(canvas) {
            return self.touched_right_wall(canvas)
        }

        self.touched_left_wall(canvas)
    }
    
    pub fn draw(&self, canvas: &mut Canvas, color: Color) {
        canvas.fill_rect(self.to_rect(), color);
    }
}