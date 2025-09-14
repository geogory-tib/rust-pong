use std::{thread, time::Duration};

use raylib::{audio::Sound, color::Color, math, prelude::RaylibDraw};

use crate::{BALL_CENTER, WINDOW_HEIGHT, WINDOW_WIDTH, delay, rand};
pub const NORMAL_SPEED: i32 = 2;
pub struct paddle {
    pub height: i32,
    pub width: i32,
    pub x: i32,
    pub y: i32,
    y2: i32,
    pub dir: i32,
    pub color: raylib::color::Color,
}
impl paddle {
    pub fn init(w: i32, h: i32, X: i32, Y: i32, clr: Color) -> paddle {
        let ret_paddle = paddle {
            height: h,
            width: w,
            x: X,
            y: Y,
            y2: Y + h,
            dir: 0,
            color: clr,
        };
        ret_paddle
    }
    pub fn paddle_ai(&mut self, game_ball: &ball) {
        if rand::rand(11) == 3 {
            return;
        }
        if game_ball.y > self.y + 40 {
            self.y += NORMAL_SPEED;
        } else if game_ball.y < self.y + 40 {
            self.y -= NORMAL_SPEED;
        } else if game_ball.y == self.y + 40 {
            return;
        }
    }
}
pub struct ball {
    pub x: i32,
    pub y: i32,
    pub radius: f32,
    pub color: raylib::color::Color,
    dirX: i32,
    dirY: i32,
}
impl ball {
    pub fn init(X: i32, Y: i32, R: f32, clr: raylib::color::Color) -> ball {
        let ret_ball = ball {
            x: X,
            y: Y,
            radius: R,
            color: clr,
            dirX: NORMAL_SPEED,
            dirY: 0,
        };
        ret_ball
    }
    pub fn update_pos(&mut self) {
        self.x += self.dirX;
        self.y += self.dirY;
    }
    pub fn check_ball_collsion(
        &mut self,
        player_paddle: &paddle,
        ai_paddle: &paddle,
        beep: &Sound,
        score_sound: &Sound,
    ) {
        if self.y == 0 {
            self.dirY = NORMAL_SPEED
        }
        if self.y == WINDOW_HEIGHT {
            self.dirY = -NORMAL_SPEED
        }
        if self.x == player_paddle.x + self.radius as i32 {
            self.dirX = NORMAL_SPEED;
            if !(self.y >= player_paddle.y && self.y <= player_paddle.y + player_paddle.height) {
                unsafe {
                    score_sound.play();
                }
                self.refresh_ball(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, NORMAL_SPEED);
                thread::sleep(Duration::new(4, 0));
                return;
            }
            beep.play();
            if self.y >= player_paddle.y && self.y <= player_paddle.y + 25 {
                self.dirY = -NORMAL_SPEED
            } else if self.y <= player_paddle.y + player_paddle.height
                && self.y >= player_paddle.y + player_paddle.height - 25
            {
                self.dirY = NORMAL_SPEED
            } else if self.y >= player_paddle.y + player_paddle.height / 2 - 15 {
                if self.y <= player_paddle.y + player_paddle.height / 2 + 15 {
                    self.dirY = 0;
                }
            }
        }
        if self.x == ai_paddle.x - self.radius as i32 {
            self.dirX = -NORMAL_SPEED;
            if !(self.y >= ai_paddle.y && self.y <= ai_paddle.y + ai_paddle.height) {
                unsafe {
                    score_sound.play();
                }
                self.refresh_ball(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, -NORMAL_SPEED);
                thread::sleep(Duration::new(4, 0));
                return;
            }
            unsafe {
                beep.play();
            }
            if self.y >= ai_paddle.y && self.y <= ai_paddle.y + 25 {
                self.dirY = NORMAL_SPEED
            } else if self.y <= ai_paddle.y + ai_paddle.height
                && self.y >= ai_paddle.y + ai_paddle.height - 25
            {
                self.dirY = NORMAL_SPEED
            } else if self.y >= ai_paddle.y + ai_paddle.height / 2 - 15 {
                if self.y <= ai_paddle.y + ai_paddle.height / 2 + 15 {
                    self.dirY = 0;
                }
            }
        }
    }
    fn refresh_ball(&mut self, X: i32, Y: i32, dir: i32) {
        self.x = X;
        self.y = Y;
        self.dirY = 0;
        self.dirX = dir;
    }
}
pub struct score_board {
    player_score: i32,
    p_x: i32,
    p_y: i32,
    ai_score: i32,
    ai_x: i32,
    ai_y: i32,
}
