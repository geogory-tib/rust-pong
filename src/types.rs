use raylib::{color::Color, prelude::RaylibDraw};

pub struct paddle {
    pub height: i32,
    pub width: i32,
    pub x: i32,
    pub y: i32,
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
            dir: 0,
            color: clr,
        };
        ret_paddle
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
            dirX: 1,
            dirY: 0,
        };
        ret_ball
    }
    pub fn update_pos(&mut self) {
        self.x += self.dirX;
        self.y += self.dirY;
    }
}
