use std::{i32, time::Duration};

use raylib::{audio, color::Color, math::Vector2, prelude::RaylibDraw};

mod input;
mod rand;
mod types;
const WINDOW_WIDTH: i32 = 680;
const WINDOW_HEIGHT: i32 = 480;
const BALL_CENTER: (i32, i32) = (WINDOW_WIDTH / 2 + 8, WINDOW_HEIGHT / 2);
const delay: u64 = 7;
fn main() {
    let mut player_paddle = types::paddle::init(10, 80, 0, WINDOW_HEIGHT / 2 - 40, Color::WHITE);
    let mut ai_paddle = types::paddle::init(
        10,
        80,
        WINDOW_WIDTH - 10,
        WINDOW_HEIGHT / 2 - 40,
        Color::WHITE,
    );
    let mut ball = types::ball::init(BALL_CENTER.0, BALL_CENTER.1, 8.0, Color::WHITE);
    let (mut ray_handle, ray_thread) = raylib::init()
        .width(WINDOW_WIDTH)
        .height(WINDOW_HEIGHT)
        .title("Rust-Pong")
        .resizable()
        .build();

    let mut ray_audio = unsafe {
        raylib::audio::RaylibAudio::init_audio_device().expect("Error initalizing Raylib Audio ")
    };

    let beep = unsafe { ray_audio.new_sound("audio/pongbeep.wav").unwrap_unchecked() };
    let score_sound = unsafe {
        ray_audio
            .new_sound("audio/pongscore.wav")
            .unwrap_unchecked()
    };
    let thread_delay = Duration::from_millis(delay);
    while !ray_handle.window_should_close() {
        let mut ray_drawer = ray_handle.begin_drawing(&ray_thread);
        ray_drawer.clear_background(Color::BLACK);
        ray_drawer.draw_circle_lines(ball.x, ball.y, ball.radius, ball.color);
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
        input::handle_input(&mut ray_drawer, &mut player_paddle);
        ball.check_ball_collsion(&player_paddle, &ai_paddle, &beep, &score_sound);
        ball.update_pos();
        ai_paddle.paddle_ai(&ball);
        std::thread::sleep(thread_delay);
    }
}
