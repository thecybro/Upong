use crate::utils;

use crate::{SelectedDifficulty, Difficulty};

impl SelectedDifficulty {
    pub const fn ball_speed_x(&self) -> f32 {
        match self.0 {
            Difficulty::Easy => 300.0,
            Difficulty::Medium => 350.0,
            Difficulty::Hard => 400.0
        }
    }

    pub const fn ball_speed_y(&self) -> f32 {
        self.ball_speed_x()
    }

    pub const fn ball_speed(&self) -> f32 {
        utils::const_sqrt(
            (self.ball_speed_x() * self.ball_speed_y())
            + 
            (self.ball_speed_x() * self.ball_speed_y())
        )
    }

    pub const fn ball_max_speed_x(&self) -> f32 {
        self.ball_speed_x() * 2.0
    }

    pub const fn ball_max_speed_y(&self) -> f32 {
        self.ball_speed_y() * 2.0
    }

    pub const fn ball_max_speed(&self) -> f32 {
        utils::const_sqrt(
            (self.ball_max_speed_x() * self.ball_max_speed_y())
            + 
            (self.ball_max_speed_x() * self.ball_max_speed_y())
        )
    }

    pub const fn player_paddle_speed(&self) -> f32 {
        450.0
    }

    pub const fn enemy_paddle_speed(&self) -> f32 {
        match self.0 {
            Difficulty::Easy => self.player_paddle_speed() * 0.5,
            Difficulty::Medium => self.player_paddle_speed() * 0.65,
            Difficulty::Hard => self.player_paddle_speed() * 0.8,
        }
    }

    pub const fn enemy_distance(&self) -> f32 {
        match self.0 {
            Difficulty::Easy => 20.0,
            Difficulty::Medium => 15.0,
            Difficulty::Hard => 10.0,
        }
    }
    
    pub const fn ball_speed_increase(&self) -> f32 {
        match self.0 {
            Difficulty::Easy => 0.2,
            Difficulty::Medium => 0.3,
            Difficulty::Hard => 0.4,
        }
    }

    pub const fn ball_speed_decrease(&self) -> f32 {
        match self.0 {
            Difficulty::Easy => 1.01,
            Difficulty::Medium => 1.02,
            Difficulty::Hard => 1.03,
        }
    }
}