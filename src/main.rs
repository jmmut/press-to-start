use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::anchorer::Anchorer;
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::{Interaction, Widget};
use juquad::widgets::{StateStyle, Style};
use juquad::PixelPosition;
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub const FORCE_RANGE_PIXELS: f32 = 100.0;
pub const LIGHT_GREEN: Color = Color::new(0.7, 0.85, 0.7, 1.0);
pub const TOOLTIP_BACKGROUND: Color = LIGHTGRAY;

pub const STYLE: Style = Style {
    at_rest: StateStyle {
        bg_color: LIGHT_GREEN,
        text_color: BLACK,
        border_color: DARKGRAY,
    },
    hovered: StateStyle {
        bg_color: Color::new(0.8, 0.9, 0.8, 1.0),
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

const DIALOG_DELAY_SECONDS: f64 = 5.0;
pub const STAGE_TORUS_DIALOGS: &[&str] = &[
    "Hey, don't scare me like that!",
    "You wanna play dirty, huh?",
];
pub const STAGE_PRISON_DIALOGS: &[&str] = &[
    "You can no longer leave this window!",
    "You can give up if you want...",
];
pub const STAGE_TORUS_ENABLED: bool = true;

#[macroquad::main(window_conf)]
async fn main() {
    let button = stage_torus().await;
    stage_prison(button).await;
}

async fn stage_torus() -> Button {
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
        if is_key_pressed(KeyCode::Escape) {
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

fn with_alpha(base: Color, alpha: f32) -> Color {
    Color::new(base.r, base.g, base.b, alpha)
}
pub fn draw_halo(x: f32, y: f32, r: f32, color: Color) {
    draw_poly(x, y, 40, r, 0., color);
}

pub struct GrabbedMouseInput {
    inner: Box<dyn InputTrait>,
    mouse_origin: Vec2,
}
impl GrabbedMouseInput {
    pub fn new(mouse_origin: Vec2) -> Self {
        Self {
            inner: Box::new(InputMacroquad),
            mouse_origin,
        }
    }
}
impl InputTrait for GrabbedMouseInput {
    fn is_key_down(&self, key: KeyCode) -> bool {
        self.inner.is_key_down(key)
    }

    fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.inner.is_key_pressed(key)
    }

    fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.inner.is_mouse_button_down(button)
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.inner.is_mouse_button_pressed(button)
    }

    fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        self.inner.is_mouse_button_released(button)
    }

    fn mouse_position(&self) -> PixelPosition {
        self.inner.mouse_position() + self.mouse_origin
    }

    fn mouse_wheel(&self) -> PixelPosition {
        self.inner.mouse_wheel()
    }

    fn clone(&self) -> Box<dyn InputTrait> {
        Box::new(GrabbedMouseInput {
            inner: self.inner.clone(),
            mouse_origin: self.mouse_origin,
        })
    }
}
fn animate_pos_to(button: &mut Button, target_pos: Vec2) {
    button.reanchor(Anchor::center_v(target_pos))
}

fn create_tooltip_anchor(sw: f32, sh: f32, next_to_this: Rect) -> Anchor {
    let center = next_to_this.center();
    let left = center.x < sw * 0.5;
    let horiz = if left {
        Horizontal::Left
    } else {
        Horizontal::Right
    };
    let top = center.y < sh * 0.5;
    let vert = if top { Vertical::Top } else { Vertical::Bottom };
    let anchor = Anchor::new(horiz, vert, center.x, center.y);
    anchor
}

fn compute_force(mouse_pos: Vec2, button_center: Vec2) -> Vec2 {
    let range = FORCE_RANGE_PIXELS;
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
pub fn new_button_grabbed(text: &str, anchor: Anchor, input: &Box<dyn InputTrait>) -> Button {
    let input_clone = (*input).clone();
    Button::new_generic(
        text,
        anchor,
        FONT_SIZE,
        None,
        macroquad::prelude::measure_text,
        input_clone,
    )
}
pub fn render_button(button: &Button) {
    button.render_default(&STYLE);
}

pub fn render_tooltip(text: &str, anchor: Anchor) {
    let text_rect = TextRect::new(&text, anchor, FONT_SIZE);
    draw_rect(text_rect.rect(), TOOLTIP_BACKGROUND);
    draw_rect_lines(text_rect.rect(), 2.0, STYLE.at_rest.border_color);
    text_rect.render_default(&STYLE.at_rest);
}
