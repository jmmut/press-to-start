use crate::render_button;
use juquad::widgets::button::Button;
use macroquad::prelude::next_frame;

pub async fn stage_rockets(mut button: Button) {
    loop {
        render_button(&button);
        next_frame().await
    }
}
