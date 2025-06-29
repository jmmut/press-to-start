use crate::{
    compute_force, create_tooltip_anchor, draw_halo, new_button, render_button, render_tooltip,
    should_quit, with_alpha, DIALOG_DELAY_SECONDS, FONT_SIZE, FORCE_RANGE_PIXELS,
    STAGE_TORUS_DIALOGS, STAGE_TORUS_ENABLED, STYLE,
};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::anchorer::Anchorer;
use juquad::widgets::button::Button;
use juquad::widgets::{Interaction, Widget};
use macroquad::color::{Color, DARKGREEN, GRAY, LIGHTGRAY};
use macroquad::input::{is_key_pressed, mouse_position, KeyCode};
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::miniquad::date::now;
use macroquad::prelude::{clear_background, draw_line, next_frame, screen_height, screen_width};

pub async fn stage_torus() -> Button {
    let (sw, sh) = (screen_width(), screen_height());
    let anchor = Anchor::center(sw * 0.5, sh * 0.5);
    let mut text = "Start";
    let mut dialog_index = 0;
    let mut start_dialogue = None;
    let mut button = new_button(text, anchor);
    let layout = Layout::Vertical {
        direction: Vertical::Bottom,
        alignment: Horizontal::Left,
    };
    let debug = false;
    let debug_field = false;
    let mut move_button = true;

    let dialogs = STAGE_TORUS_DIALOGS;
    loop {
        if !STAGE_TORUS_ENABLED {
            break;
        }
        if should_quit() {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            println!("mouse pos: {:?}", mouse_position());
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
                let anchor = Anchor::bottom_left_v(mouse_pos);
                render_tooltip(&format!("force: {}", force), anchor);
            }
        }
        let displacement = compute_force(mouse_pos, button_center);
        let new_pos = button_center + displacement;

        if move_button {
            button.reanchor(Anchor::center_v(new_pos));
        }
        let diff = (mouse_pos - button_center).length();
        let alpha =
            ((2.0 * FORCE_RANGE_PIXELS - diff) / (2.0 * FORCE_RANGE_PIXELS) / 3.0).clamp(0.0, 0.5);
        // 2*RANGE => 0, 0.5*range => 1
        let halo_color = with_alpha(GRAY, alpha);
        draw_halo(new_pos.x, new_pos.y, diff, halo_color);
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

        // check if any alternative buttons have interactions
        let original = button.rect();
        let mut interaction = button.interact();
        for extra in &extra_buttons {
            if interaction != Interaction::None {
                break;
            }
            *button.rect_mut() = *extra;
            interaction = button.interact();
        }
        // any extra button with interaction will become the main button
        let rect_interacted = if interaction != Interaction::None {
            Some(button.rect())
        } else {
            None
        };

        // render main and extra buttons
        *button.rect_mut() = original;
        render_button(&button);
        for extra in &extra_buttons {
            *button.rect_mut() = *extra;
            render_button(&button);
        }

        // print dialogue if triggered
        if let Some(start_dialogue_ts) = start_dialogue {
            let current_ts = now();
            if current_ts - start_dialogue_ts > DIALOG_DELAY_SECONDS {
                start_dialogue = None;
                if dialog_index == dialogs.len() - 1 {
                    break;
                }
                dialog_index = (dialog_index + 1).min(dialogs.len() - 1);
            } else {
                let anchor = create_tooltip_anchor(sw, sh, button.rect());
                render_tooltip(dialogs[dialog_index], anchor);
                //         let anchor = center + match (horiz, vert) {
                //     (Horizontal::Left, Vertical::Top) => Anchor::new(),
                //     (Horizontal::Left, Vertical::Bottom) => 5.0,
                //     (Horizontal::Right, Vertical::Top) => -5.0,
                //     (Horizontal::Right, Vertical::Bottom) => -5.0,
                //     _ => panic!(),
                // };
            }
        }

        // start dialogue with interacted
        *button.rect_mut() = original;
        if let Some(interacted) = rect_interacted {
            if start_dialogue.is_some() {
                let prev_dialog_index = dialog_index;
                dialog_index = (dialog_index + 1).min(dialogs.len() - 1);
                if prev_dialog_index != dialog_index {
                    start_dialogue = Some(now());
                }
            } else {
                start_dialogue = Some(now());
            }
            *button.rect_mut() = interacted;
        } else {
            let screen = Rect::new(0.0, 0.0, sw, sh);
            if screen.intersect(button.rect()).is_none() {
                // no interaction and main button went out, so choose an extra as main
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

        // should not happen
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
        next_frame().await;
    }
    button
}
