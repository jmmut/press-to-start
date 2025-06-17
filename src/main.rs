use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::{interact, Widget};
use macroquad::prelude::*;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::{StateStyle, Style};
use juquad::widgets::anchorer::Anchorer;
use juquad::widgets::button_group::LabelGroup;
use juquad::widgets::text::TextRect;

pub const STYLE: Style = Style {
    at_rest: StateStyle {
        bg_color: LIGHTGRAY,
        text_color: BLACK,
        border_color: DARKGRAY,
    },
    hovered: StateStyle {
        bg_color: WHITE,
        text_color: BLACK,
        border_color: LIGHTGRAY,
    },
    pressed: StateStyle {
        bg_color: GRAY,
        text_color: WHITE,
        border_color: DARKGRAY,
    },
};

#[macroquad::main("press-to-start")]
async fn main() {
    let mut text = "Start";
    let input: Box<dyn InputTrait> = Box::new(InputMacroquad);
    let (sw, sh) = (screen_width(), screen_height());
    let anchor = Anchor::center(sw * 0.5, sh * 0.5);
    let mut button = new_button(text, anchor);
    let layout = Layout::Vertical { direction: Vertical::Bottom, alignment: Horizontal::Left };
    let top_left = Anchor::top_left(0.0, 0.0);
    // let labels = LabelGroup::new(FONT_SIZE, top_left);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        let mouse_pos = Vec2::from(mouse_position());
        let interaction = interact(button.rect(), &input);
        // if interaction.is_hovered() {
        let button_center = button.rect().center();
        let diff = button_center - mouse_pos;
        let diff_unit = diff.normalize_or_zero();
        let magnitude = diff.length_squared();
        let range = 10000.0;
        let force = (range - magnitude.min(range)).clamp(0.0, range) * 0.01;
        let displacement = diff_unit * force;
        let new_pos = button_center + displacement;
        
        button.reanchor(Anchor::center_v(new_pos));
        let mut anchorer = Anchorer::new_pos(layout, vec2(0.0, 0.0), 0.0);
        for t in [
            &format!("diff: {}", diff),
            &format!("magnitude: {}", magnitude),
            &format!("force: {}", force),
            &format!("displacement: {}", displacement),
        ] {
            anchorer.new_text(t, FONT_SIZE).render_default(&STYLE.at_rest);
        }
        if interaction.is_clicked() {
            text = "Start again";
        }
        button.render_default(&STYLE);
        draw_line(button_center.x, button_center.y, new_pos.x, new_pos.y, 2.0, DARKGREEN);
        next_frame().await
    }
}

pub const FONT_SIZE: f32 = 16.0;

pub fn new_button(text: &str, anchor: Anchor) -> Button {
    Button::new(text, anchor, FONT_SIZE)
}