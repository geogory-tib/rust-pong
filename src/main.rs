use std::{i32, time::Duration};

use raylib::{color::Color, prelude::RaylibDraw};

mod types;

const WINDOW_WIDTH: i32 = 680;
const WINDOW_HEIGHT: i32 = 480;

fn main() {
    let mut player_paddle = types::paddle::init(10, 80, 0, WINDOW_HEIGHT / 2, Color::WHITE);
    let mut ai_paddle =
        types::paddle::init(10, 80, WINDOW_WIDTH - 10, WINDOW_HEIGHT / 2, Color::WHITE);
    let mut ball = types::ball::init(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, 8.0, Color::WHITE);
    let (mut ray_handle, ray_thread) = raylib::init()
        .width(WINDOW_WIDTH)
        .height(WINDOW_HEIGHT)
        .title("R-Pong")
        .build();
    let thread_delay = Duration::from_millis(16);
    while !ray_handle.window_should_close() {
        let mut ray_drawer = ray_handle.begin_drawing(&ray_thread);
        ray_drawer.clear_background(Color::BLACK);
        ray_drawer.draw_circle(ball.x, ball.y, ball.radius, ball.color);
        ray_drawer.draw_rectangle(
            player_paddle.x,
            player_paddle.y,
            player_paddle.width,
            player_paddle.height,
            player_paddle.color,
        );
        ray_drawer.draw_rectangle(
            ai_paddle.x,
            ai_paddle.y,
            ai_paddle.width,
            ai_paddle.height,
            ai_paddle.color,
        );
        ball.update_pos();
        std::thread::sleep(thread_delay);
    }
}
