use crate::{compute_force, compute_force_towards, render_button, should_quit};
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::Widget;
use macroquad::color::LIGHTGRAY;
use macroquad::input::KeyCode;
use macroquad::prelude::{
    clear_background, is_key_pressed, mouse_position, next_frame, screen_height, screen_width,
    vec2, Vec2,
};

pub async fn stage_rockets(mut button: Button) {
    loop {
        if should_quit() {
            break;
        }
        let (sw, sh) = (screen_width(), screen_height());
        let screen_center = vec2(sw * 0.5, sh * 0.5);
        if is_key_pressed(KeyCode::R) {
            button.reanchor(Anchor::center_v(screen_center));
        }

        let mouse_pos = Vec2::from(mouse_position());
        let button_center = button.rect().center();
        let displacement = compute_force_towards(mouse_pos, button_center, screen_center);
        let new_pos = button_center + displacement;
        button.reanchor(Anchor::center_v(new_pos));

        clear_background(LIGHTGRAY);
        render_button(&button);
        next_frame().await
    }
}
