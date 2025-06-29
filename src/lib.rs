use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::anchor::{Anchor, Horizontal, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::{StateStyle, Style, Widget};
use juquad::PixelPosition;
use macroquad::color::{Color, BLACK, DARKGRAY, GRAY, LIGHTGRAY, WHITE};
use macroquad::input::{is_key_pressed, KeyCode, MouseButton};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{draw_poly, vec2};

pub mod stages {
    pub mod prison;
    pub mod rockets;
    pub mod torus;
}

pub const STAGE_TORUS_ENABLED: bool = false;

const DIALOG_DELAY_SECONDS: f64 = 5.0;
pub const STAGE_TORUS_DIALOGS: &[&str] =
    &["Hey, don't scare me like that!", "You wanna play, huh?"];
pub const STAGE_PRISON_DIALOGS: &[&str] = &[
    "You can no longer leave this window!",
    "You can give up if you want...",
];

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

fn should_quit() -> bool {
    is_key_pressed(KeyCode::Escape)
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
fn compute_force_towards(mouse_pos: Vec2, button_center: Vec2, target: Vec2) -> Vec2 {
    let range = FORCE_RANGE_PIXELS;
    let diff = button_center - mouse_pos;
    let diff_unit = diff.normalize_or_zero();
    let left_diff = vec2(diff_unit.y, -diff_unit.x);
    let to_target = (target - button_center).normalize_or_zero();
    let magnitude = diff.length();
    let force = (range - magnitude).max(0.0);
    let displacement = diff_unit * force * force * 0.01;
    let cos = to_target.dot((mouse_pos - button_center).normalize_or_zero());
    let sideways = 1.0 - cos.abs();
    let sideways_displacement = left_diff * force * force * 0.01;

    displacement * sideways + (1.0 - sideways) * sideways_displacement
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
