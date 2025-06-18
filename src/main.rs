use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::anchorer::Anchorer;
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::{Interaction, Widget};
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

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
const DEFAULT_WINDOW_TITLE: &str = "Press to Start";

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let (sw, sh) = (screen_width(), screen_height());
    let anchor = Anchor::center(sw * 0.5, sh * 0.5);
    let mut text = "Start";
    let mut button = new_button(text, anchor);
    let layout = Layout::Vertical {
        direction: Vertical::Bottom,
        alignment: Horizontal::Left,
    };
    let debug = false;
    let debug_field = false;
    let mut move_button = true;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        if debug {
            let mut toggle_move =
                new_button("toggle move", Anchor::top_center(sw * 0.5, sh * 0.25));
            if toggle_move.interact().is_clicked() {
                move_button = !move_button;
            }
            render_button(&toggle_move);
        }
        let mouse_pos = Vec2::from(mouse_position());
        let button_center = button.rect().center();
        let n_x = 50;
        let n_y = 50;
        if debug_field {
            let mut tooltip = None;
            for i_x in 0..n_x {
                for i_y in 0..n_y {
                    let pos = vec2(i_x as f32 * sw / n_x as f32, i_y as f32 * sh / n_y as f32);
                    let displacement = compute_force(pos, button_center);
                    let force = displacement.length();
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
        let displacement = compute_force(mouse_pos, button_center);
        let new_pos = button_center + displacement;

        if move_button {
            button.reanchor(Anchor::center_v(new_pos));
        }
        let size = button.rect().size();
        let mut extra_buttons = Vec::new();
        if button.rect().x < 0.0 {
            let extra = Anchor::center_v(new_pos + vec2(sw, 0.0)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().y < 0.0 {
            let extra = Anchor::center_v(new_pos + vec2(0.0, sh)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().right() > sw {
            let extra = Anchor::center_v(new_pos + vec2(-sw, 0.0)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().bottom() > sh {
            let extra = Anchor::center_v(new_pos + vec2(0.0, -sh)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().x < 0.0 && button.rect().y < 0.0 {
            let extra = Anchor::center_v(new_pos + vec2(sw, sh)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().y < 0.0 && button.rect().right() > sw {
            let extra = Anchor::center_v(new_pos + vec2(-sw, sh)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().right() > sw && button.rect().bottom() > sh {
            let extra = Anchor::center_v(new_pos + vec2(-sw, -sh)).get_rect(size);
            extra_buttons.push(extra);
        }
        if button.rect().bottom() > sh && button.rect().x < 0.0 {
            let extra = Anchor::center_v(new_pos + vec2(sw, -sh)).get_rect(size);
            extra_buttons.push(extra);
        }
        // move_inside(button.rect_mut(), Rect::new(0.0, 0.0, sw, sh));
        if debug {
            let mut anchorer = Anchorer::new_pos(layout, vec2(0.0, 0.0), 0.0);

            for t in [
                // &format!("diff: {}", diff),
                // &format!("diff.abs(): {}", diff.abs()),
                // &format!("complementary: {}", complementary),
                // &format!("clamped: {}", clamped),
                // &format!("magnitude: {}", magnitude),
                // &format!("force: {}", force),
                // &format!("displacement: {}", displacement),
                &format!("button center: {}", button_center),
            ] {
                let text_rect = anchorer.new_text(t, FONT_SIZE);
                draw_rect(text_rect.rect(), LIGHTGRAY);
                text_rect.render_default(&STYLE.at_rest);
            }
        }
        let original = button.rect();
        let mut interaction = button.interact();
        for extra in &extra_buttons {
            if interaction != Interaction::None {
                break;
            }
            *button.rect_mut() = *extra;
            interaction = button.interact();
        }
        let rect_interacted = if interaction != Interaction::None {
            Some(button.rect())
        } else {
            None
        };
        *button.rect_mut() = original;
        render_button(&button);
        for extra in &extra_buttons {
            *button.rect_mut() = *extra;
            render_button(&button);
        }
        *button.rect_mut() = original;
        if let Some(interacted) = rect_interacted {
            *button.rect_mut() = interacted;
        } else {
            let screen = Rect::new(0.0, 0.0, sw, sh);
            if screen.intersect(button.rect()).is_none() {
                let mut max_area = 0.0;
                for extra in extra_buttons {
                    if let Some(intersection) = extra.intersect(screen) {
                        let area = intersection.w * intersection.h;
                        if area > max_area {
                            max_area = area;
                            *button.rect_mut() = extra;
                        }
                    }
                }
            }
        }
        if interaction.is_clicked() {
            text = "Start again";
            button = new_button(text, anchor);
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

fn compute_force(mouse_pos: Vec2, button_center: Vec2) -> Vec2 {
    let range = 100.0;
    let diff = button_center - mouse_pos;
    let diff_unit = diff.normalize_or_zero();
    let magnitude = diff.length();
    let force = (range - magnitude).max(0.0);
    // let range = 100.0;
    // let complementary = vec2(range, range) - diff.abs();
    // let clamped = vec2(complementary.x.max(0.0), complementary.y.max(0.0));
    // let force = clamped.length_squared();
    let displacement = diff_unit * force * force * 0.01;
    displacement
}

#[allow(unused)]
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
