use crate::{new_button, render_button, should_quit, FONT_SIZE};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::text::TextRect;
use juquad::widgets::{StateStyle, Widget};
use macroquad::color::{Color, LIGHTGRAY, RED};
use macroquad::input::KeyCode;
use macroquad::prelude::{clear_background, is_key_pressed, next_frame, Vec2};

pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const GAME_OVER_STYLE: StateStyle = StateStyle {
    bg_color: TRANSPARENT,
    text_color: RED,
    border_color: TRANSPARENT,
};
pub enum AfterGameOver {
    RestartStage,
    Quit,
}
pub async fn stage_game_over(sw: f32, sh: f32, screen_center: Vec2) -> AfterGameOver {
    let anchor = Anchor::center(sw * 0.5, sh * 0.25);
    let text_rect = TextRect::new("YOU DIED", anchor, FONT_SIZE * 5.0);
    let mut bg_color = LIGHTGRAY;
    let almost_black = Color::new(0.2, 0.2, 0.2, 1.0);
    loop {
        clear_background(bg_color);
        bg_color.r -= 0.007;
        bg_color.g -= 0.007;
        bg_color.b -= 0.007;
        if bg_color.r < 0.2 {
            break;
        }
        draw_rect(text_rect.rect(), almost_black);
        text_rect.render_default(&GAME_OVER_STYLE);
        next_frame().await;
    }
    let anchor = Anchor::center_v(screen_center);
    let mut exit = new_button("Exit", anchor);
    loop {
        if is_key_pressed(KeyCode::R) {
            return AfterGameOver::RestartStage;
        }
        if should_quit() || exit.interact().is_clicked() {
            return AfterGameOver::Quit;
        }
        clear_background(almost_black);
        draw_rect(text_rect.rect(), almost_black);
        text_rect.render_default(&GAME_OVER_STYLE);
        render_button(&exit);
        next_frame().await;
    }
}
