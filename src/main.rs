use juquad::draw::draw_rect;
use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::anchorer::Anchorer;
use juquad::widgets::button::Button;
use juquad::widgets::button_group::LabelGroup;
use juquad::widgets::text::TextRect;
use juquad::widgets::{interact, Widget};
use juquad::widgets::{StateStyle, Style};
use macroquad::prelude::*;

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
    let (sw, sh) = (screen_width(), screen_height());
    let anchor = Anchor::center(sw * 0.5, sh * 0.5);
    let mut text = "Start";
    let mut button = new_button(text, anchor);
    let layout = Layout::Vertical {
        direction: Vertical::Bottom,
        alignment: Horizontal::Left,
    };
    let top_left = Anchor::top_left(0.0, 0.0);
    // let labels = LabelGroup::new(FONT_SIZE, top_left);
    let debug = false;
    let move_button = true;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        let mouse_pos = Vec2::from(mouse_position());
        // if interaction.is_hovered() {
        let button_center = button.rect().center();
        let n_x = 50;
        let n_y = 50;
        if debug {
            let mut tooltip = None;
            for i_x in 0..n_x {
                for i_y in 0..n_y {
                    let pos = vec2(i_x as f32 * sw / n_x as f32, i_y as f32 * sh / n_y as f32);
                    let (_, _, _, force) = compute_force(pos, button_center);
                    let pad = 10.0;
                    let max = 10000.0;
                    let normalized = force.min(max).log10() / max.log10();
                    let color = Color::new(normalized, normalized, normalized, 1.0);
                    let rect = Rect::new(pos.x - pad, pos.y - pad, pad, pad);
                    draw_rect(rect, color);
                    if rect.contains(mouse_pos) {
                        tooltip = Some(force);
                    }
                }
            }
            if let Some(force) = tooltip {
                let text_rect = TextRect::new(
                    &format!("force: {}", force),
                    Anchor::bottom_left_v(mouse_pos),
                    FONT_SIZE,
                );
                draw_rect(text_rect.rect(), LIGHTGRAY);
                text_rect.render_default(&STYLE.at_rest);
            }
        }
        let (diff, complementary, clamped, force) = compute_force(mouse_pos, button_center);
        // let force = (range - magnitude.min(range)).clamp(0.0, range) * 0.01;
        let diff_unit = diff.normalize_or_zero();
        let displacement = diff_unit * force * 0.01;
        let new_pos = button_center + displacement;

        if move_button {
            button.reanchor(Anchor::center_v(new_pos));
        }
        let mut extra_buttons = Vec::new();
        if button.rect().x < 0.0 {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(sw, 0.0)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().y < 0.0 {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(0.0, sh)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().right() > sw {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(-sw, 0.0)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().bottom() > sh {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(0.0, -sh)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().x < 0.0 && button.rect().y < 0.0 {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(sw, sh)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().y < 0.0 && button.rect().right() > sw {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(-sw, sh)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().right() > sw && button.rect().bottom() > sh {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(-sw, -sh)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        if button.rect().bottom() > sh && button.rect().x < 0.0 {
            let extra = new_button(text, Anchor::center_v(new_pos + vec2(sw, -sh)));
            render_button(&extra);
            extra_buttons.push(extra);
        }
        // move_inside(button.rect_mut(), Rect::new(0.0, 0.0, sw, sh));
        if debug {
            let mut anchorer = Anchorer::new_pos(layout, vec2(0.0, 0.0), 0.0);

            for t in [
                &format!("diff: {}", diff),
                &format!("diff.abs(): {}", diff.abs()),
                &format!("complementary: {}", complementary),
                &format!("clamped: {}", clamped),
                // &format!("magnitude: {}", magnitude),
                &format!("force: {}", force),
                &format!("displacement: {}", displacement),
            ] {
                let text_rect = anchorer.new_text(t, FONT_SIZE);
                draw_rect(text_rect.rect(), LIGHTGRAY);
                text_rect.render_default(&STYLE.at_rest);
            }
        }
        if button.interact().is_clicked() {
            text = "Start again";
            button = new_button(text, anchor);
        }
        for extra in &mut extra_buttons {
            if extra.interact().is_clicked() {
                text = "Start again";
                button = new_button(text, anchor);
            }
            render_button(&extra);
        }
        render_button(&button);
        let screen = Rect::new(0.0, 0.0, sw, sh);
        if screen.intersect(button.rect()).is_none() {
            let mut max_area = 0.0;
            for extra in extra_buttons {
                if let Some(intersection) = extra.rect().intersect(screen) {
                    let area = intersection.w * intersection.h;
                    if area > max_area {
                        max_area = area;
                        button = extra;
                    }
                }
            }
        }
        if debug {
            draw_line(
                button_center.x,
                button_center.y,
                new_pos.x,
                new_pos.y,
                2.0,
                DARKGREEN,
            );
        }
        next_frame().await
    }
}

fn compute_force(mouse_pos: Vec2, button_center: Vec2) -> (Vec2, Vec2, Vec2, f32) {
    let diff = button_center - mouse_pos;
    let diff_unit = diff.normalize_or_zero();
    let magnitude = diff.length();
    let force = (100.0 - magnitude).max(0.0);
    let range = 100.0;
    let complementary = vec2(range, range) - diff.abs();
    let clamped = vec2(complementary.x.max(0.0), complementary.y.max(0.0));
    // let force = clamped.length_squared();
    (diff, complementary, clamped, force * force)
}

fn move_inside(rect: &mut Rect, container: Rect) {
    rect.x += (container.x - rect.x).max(0.0);
    rect.y += (container.y - rect.y).max(0.0);
    rect.x -= (rect.right() - container.right()).max(0.0);
    rect.y -= (rect.bottom() - container.bottom()).max(0.0);
}

pub const FONT_SIZE: f32 = 16.0;

pub fn new_button(text: &str, anchor: Anchor) -> Button {
    Button::new(text, anchor, FONT_SIZE)
}
pub fn render_button(button: &Button) {
    button.render_default(&STYLE);
}
