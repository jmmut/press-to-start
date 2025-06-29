use juquad::draw::draw_rect_lines;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::Widget;
use macroquad::miniquad::date::now;
use macroquad::prelude::{clear_background, is_key_pressed, mouse_position, next_frame, screen_height, screen_width, set_cursor_grab, vec2, KeyCode, Rect, Vec2, DARKPURPLE, LIGHTGRAY};
use crate::{animate_pos_to, create_tooltip_anchor, new_button_grabbed, render_button, render_tooltip, GrabbedMouseInput, DIALOG_DELAY_SECONDS, STAGE_PRISON_DIALOGS};

pub async fn stage_prison(mut button: Button) {
    let dialogs = STAGE_PRISON_DIALOGS;
    let mut dialog_index = 0;

    let fake_mouse_origin = Vec2::from(mouse_position());
    let input_grabbed: Box<dyn InputTrait> = Box::new(GrabbedMouseInput::new(fake_mouse_origin));
    println!(
        "mouse pos: {:?}, corrected: {:?}",
        mouse_position(),
        input_grabbed.mouse_position()
    );
    // set_cursor_grab(true);
    println!(
        "mouse pos after grab: {:?}, corrected: {:?}",
        mouse_position(),
        input_grabbed.mouse_position()
    );

    let stage_2_start_ts = now();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            set_cursor_grab(false);
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            println!(
                "mouse pos: {:?}, corrected: {:?}",
                mouse_position(),
                input_grabbed.mouse_position()
            );
        }
        let (sw, sh) = (screen_width(), screen_height());
        let screen_rect = Rect::new(1.0, 1.0, sw - 2.0, sh - 2.0);
        clear_background(LIGHTGRAY);
        let stage_2_duration = now() - stage_2_start_ts;
        let thickness = if stage_2_duration < 1.0 {
            stage_2_duration * 40.0
        } else if stage_2_duration < 2.0 {
            (2.0 - stage_2_duration) * 32.0 + 8.0
        } else {
            8.0
        } as f32;
        draw_rect_lines(screen_rect, thickness, DARKPURPLE);
        animate_pos_to(&mut button, vec2(sw * 0.75, sh * 0.25));
        render_button(&button);
        if stage_2_duration < DIALOG_DELAY_SECONDS {
            dialog_index = 0;
            let anchor = create_tooltip_anchor(sw, sh, button.rect());
            render_tooltip(dialogs[dialog_index], anchor);
        } else if stage_2_duration < DIALOG_DELAY_SECONDS * 1.25 {
            // wait
        } else {
            dialog_index = 1;
            let anchor = create_tooltip_anchor(sw, sh, button.rect());
            render_tooltip(dialogs[dialog_index], anchor);
            let give_up_anchor = Anchor::center_v(screen_rect.center());
            let mut give_up = new_button_grabbed("Give up", give_up_anchor, &input_grabbed);
            if give_up.interact().is_clicked() {
                set_cursor_grab(false);
                let mut bg_color = LIGHTGRAY;
                loop {
                    clear_background(bg_color);
                    bg_color.r -= 0.01;
                    bg_color.g -= 0.01;
                    bg_color.b -= 0.01;
                    if bg_color.r < 0.2 {
                        break;
                    }
                    next_frame().await;
                }
                loop {
                    let mut restart = new_button_grabbed("Restart", give_up_anchor, &input_grabbed);
                    if restart.interact().is_clicked() {
                        return;
                    }
                    render_button(&restart);
                    next_frame().await;
                }
            }
            render_button(&give_up);
        }
        next_frame().await;
    }
}