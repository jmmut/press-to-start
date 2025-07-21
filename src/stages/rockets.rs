use crate::stages::game_over::{stage_game_over, AfterGameOver};
use crate::stages::game_won::{stage_game_won, AfterGameWon};
use crate::{compute_force_towards, render_button, should_quit};
use juquad::draw::draw_rect_lines;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::Widget;
use macroquad::color::{LIGHTGRAY, WHITE};
use macroquad::input::KeyCode;
use macroquad::prelude::{clear_background, draw_triangle, draw_triangle_lines, is_key_pressed, mouse_position, next_frame, screen_height, screen_width, vec2, Rect, Vec2, RED, SKYBLUE};

#[derive(PartialEq)]
pub struct Rocket {
    pos: Vec2,
    dir: Vec2,
}

const ROCKET_SPEED: f32 = 10.0;
const ROCKET_RENDER_WIDTH: f32 = 15.0;
const ROCKET_RENDER_LENGTH: f32 = 40.0;
const MOUSE_SIZE: Vec2 = vec2(15.0, 20.0);

pub async fn stage_rockets(mut button: Button) {
    let mut rocket: Option<Rocket> = None;
    loop {
        if should_quit() {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            println!("mouse pos: {:?}", mouse_position());
        }
        let (sw, sh) = (screen_width(), screen_height());
        let screen_center = vec2(sw * 0.5, sh * 0.5);
        if is_key_pressed(KeyCode::R) {
            button.reanchor(Anchor::center_v(screen_center));
            rocket = None;
        }

        let mouse_pos = Vec2::from(mouse_position());
        let button_center = button.rect().center();
        let displacement = compute_force_towards(mouse_pos, button_center, screen_center);
        let new_pos = button_center + displacement;
        button.reanchor(Anchor::center_v(new_pos));

        if let Some(rocket) = rocket.as_mut() {
            rocket.pos += rocket.dir;
            rocket.dir += (mouse_pos - rocket.pos).normalize_or_zero() * 3.0;
            rocket.dir += -(button_center - rocket.pos).normalize_or_zero() * 2.3;
            rocket.dir = rocket.dir.normalize_or_zero() * ROCKET_SPEED;
        } else /*if button_center */{
            rocket = Some(Rocket {
                pos: button_center - vec2(0.0, button.rect().h),
                dir: vec2(0.0, -1.0),
            });
        }

        let mouse_rect = Rect::new(
            mouse_pos.x - 2.0,
            mouse_pos.y - 2.0,
            MOUSE_SIZE.x,
            MOUSE_SIZE.y,
        );
        if collide_rocket(&rocket, mouse_rect) {
            match stage_game_over(sw, sh, screen_center).await {
                AfterGameOver::RestartStage => {}
                AfterGameOver::Quit => {
                    return;
                }
            }
        }
        if collide_rocket(&rocket, button.rect()) {
            match stage_game_won(sw, sh, screen_center).await {
                AfterGameWon::RestartStage => {}
                AfterGameWon::Quit => {
                    return;
                }
            }
        }

        clear_background(LIGHTGRAY);
        draw_rect_lines(mouse_rect, 4.0, SKYBLUE);
        render_button(&button);
        render_rocket(&rocket);
        next_frame().await
    }
}

fn render_rocket(rocket: &Option<Rocket>) {
    if let Some(rocket) = rocket {
        //let render_radius = ROCKET_RENDER_WIDTH;
        // draw_circle(rocket.pos.x, rocket.pos.y, render_radius, RED);
        let dir_norm = rocket.dir.clone().normalize_or_zero();
        let to_front = dir_norm * ROCKET_RENDER_LENGTH * 0.5;
        let to_left = vec2(dir_norm.y, -dir_norm.x) * ROCKET_RENDER_WIDTH * 0.5;
        let front = rocket.pos + to_front;
        let back = rocket.pos - to_front;
        let left = rocket.pos + to_left;
        let right = rocket.pos - to_left;
        let left_wing = rocket.pos - to_front*1.2 + to_left;
        let right_wing = rocket.pos - to_front*1.2 - to_left;
        //draw_triangle(left_wing, right_wing, rocket.pos, WHITE);
        draw_triangle(left_wing, back, rocket.pos, WHITE);
        draw_triangle_lines(left_wing, back, rocket.pos, 2.0, RED);
        draw_triangle(right_wing, rocket.pos, back, WHITE);
        draw_triangle_lines(right_wing, rocket.pos, back, 2.0, RED);
        draw_triangle(front, left, right, RED);
        draw_triangle(back, right, left, RED);
    }
}

fn collide_rocket(rocket: &Option<Rocket>, target: Rect) -> bool {
    if let Some(rocket) = rocket {
        let dir_norm = rocket.dir.clone().normalize_or_zero();
        let front = rocket.pos + dir_norm * ROCKET_RENDER_LENGTH * 0.5;
        target.contains(front)
    } else {
        false
    }
}
