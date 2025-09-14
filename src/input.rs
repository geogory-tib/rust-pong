use crate::types::NORMAL_SPEED;
use crate::types::paddle;
use raylib::ffi::KeyboardKey;
pub fn handle_input(
    draw_handle: &mut raylib::drawing::RaylibDrawHandle,
    player_padde: &mut paddle,
) {
    if draw_handle.is_key_down(KeyboardKey::KEY_UP) {
        player_padde.y -= NORMAL_SPEED;
    }
    if draw_handle.is_key_down(KeyboardKey::KEY_DOWN) {
        player_padde.y += NORMAL_SPEED;
    }
    if draw_handle.is_key_down(KeyboardKey::KEY_P) {}
}
