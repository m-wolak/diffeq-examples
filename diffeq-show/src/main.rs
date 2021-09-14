use std::iter::successors;

use macroquad::prelude::*;

const DELTA: f32 = 0.01;
#[macroquad::main("Test-o-matic")]
async fn main() {
    let mut click_pos = (0.0, 0.0);
    let mut active: bool = false;

    let w = screen_width();
    let h = screen_height();
    let mut ar = (w as f32) / (h as f32);

    let cam = Camera2D::from_display_rect(Rect {
        x: -3.0 * ar,
        y: 3.0,
        w: 6.0 * ar,
        h: -6.0,
    });
    set_camera(&cam);
    loop {
        let w = screen_width();
        let h = screen_height();

        let new_ar = (w as f32) / (h as f32); // check if we need to update the camera
        if new_ar != ar {
            let cam = Camera2D::from_display_rect(Rect {
                x: -3.0 * ar,
                y: -3.0,
                w: 6.0 * ar,
                h: 6.0,
            });
            set_camera(&cam);
            ar = new_ar;
        }
        clear_background(WHITE);

        if is_mouse_button_down(MouseButton::Left) {
            click_pos = mouse_position();
            active = true;
        }
        let max_x = cam
            .screen_to_world(Vec2::new(screen_width(), screen_height()))
            .x;
        let min_x = cam.screen_to_world(Vec2::ZERO).x;
        if active {
            let world_pt = cam.screen_to_world(click_pos.into());

            let w_x = world_pt.x;
            let w_y = world_pt.y;
            draw_circle(w_x, w_y, 0.05, RED);

            let forward_points = calc_pts(rk4_step, |x, y| y.exp() - x, w_x, w_y, DELTA, max_x);
            let forward_points2 = calc_pts(forward_euler_step, |x, y| y.exp() - x, w_x, w_y, DELTA, max_x);
            for i in 1..(forward_points.len()) {
                let (ox, oy) = forward_points[i - 1];
                let (wx, wy) = forward_points[i];
                draw_line(ox, oy, wx, wy, 0.01, BLUE);
                draw_circle(forward_points2[i].0, forward_points2[i].1, 0.01, GREEN);
            }

            let backwards_points: Vec<(f32, f32)> =
                calc_pts(rk4_step, |x, y| y.exp() - x, w_x, w_y, -1.0 * DELTA, min_x);
            for i in 1..(backwards_points.len()) {
                let (ox, oy) = backwards_points[i - 1];
                let (wx, wy) = backwards_points[i];
                draw_line(ox, oy, wx, wy, 0.01, BLUE)
            }
        }
        let drawstep = (max_x - min_x) / 1000.0;
        for i in 0..1000 {
            let last_x = (i as f32) * drawstep;
            let this_x = ((i + 1) as f32) * drawstep;
            draw_line(last_x, last_x.ln(), this_x, this_x.ln(), 0.01, BLACK);
        }

        next_frame().await
    }
}

fn calc_pts(
    stepper: fn(f32, f32, fn(f32, f32) -> f32, f32) -> (f32, f32),
    f: fn(f32, f32) -> f32,
    x_0: f32,
    y_0: f32,
    delta: f32,
    x_end: f32,
) -> Vec<(f32, f32)> {
    assert!(delta * (x_end - x_0) > 0.0); // if delta is positive, x_end should be greater than x_0. If delta is negative, x_end should be less than x_0
    let stream = successors(Some((x_0, y_0)), move |(x, y)| {
        Some(stepper(*x, *y, f, delta))
    });
    if delta > 0.0 {
        stream.take_while(|(x, _)| *x <= x_end).collect()
    } else {
        stream.take_while(|(x, _)| *x >= x_end).collect()
    }
}

fn forward_euler_step(last_x: f32, last_y: f32, f: fn(f32, f32) -> f32, delta: f32) -> (f32, f32) {
    let next_y = last_y + delta * f(last_x, last_y);
    let next_x = last_x + delta;
    (next_x, next_y)
}

fn rk4_step(last_x: f32, last_y: f32, f: fn(f32, f32) -> f32, delta: f32) -> (f32, f32) {
    let k1 = f(last_x, last_y);
    let h2 = delta / 2.0;
    let x2 = last_x + h2;
    let k2 = f(last_x, last_y + h2 * k1);
    let k3 = f(x2, last_y + k2 * h2);
    let new_x = last_x + delta;
    let k4 = f(new_x, last_y + k3 * delta);
    let new_y = last_y + (1.0 / 6.0) * delta * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
    (new_x, new_y)
}
