use std::iter::successors;

use macroquad::prelude::*;
use macroquad::ui::*;

// This is the function that matters to us: Euler's method for approximating a

fn forward_euler_step(last_x: f32, last_y: f32, f: fn(f32, f32) -> f32, delta: f32) -> (f32, f32) {
    let next_y = last_y + delta * f(last_x, last_y);
    let next_x = last_x + delta;
    (next_x, next_y)
}

#[macroquad::main("Test-o-matic")]
async fn main() {
    let mut delta_inv: f32 = 10.0;

    let w = screen_width();
    let h = screen_height();
    let mut ar = (w as f32) / (h as f32);
    let cam = Camera2D::from_display_rect(Rect {
        x: -1.5 * ar,
        y: -3.0,
        w: 3.0 * ar,
        h: 3.0,
    });

    

    let mut label_str = "".to_string();
    set_camera(&cam);
    loop {
        
        let w = screen_width();
        let h = screen_height();
        
        let new_ar = (w as f32) / (h as f32);
        if new_ar != ar {
            let cam = Camera2D::from_display_rect(Rect {
                x: -1.5 * ar,
                y: -3.0,
                w: 3.0 * ar,
                h: 3.0,
            });
            set_camera(&cam);
            ar = new_ar;
        }
        clear_background(WHITE);

        let world_pt = vec2(0.0, -1.0);

        draw_line(0.0, 0.0, 0.0, -10.0, 0.01, BLACK);
        draw_line(-10.0, 0.0, 10.0, 0.0, 0.01, BLACK);

        draw_line(1.0, 0.0, 1.0, -10.0, 0.01, GREEN);
        draw_line(
            -10.0,
            -1.0 * 1.0_f32.exp(),
            10.0,
            -1.0 * 1.0_f32.exp(),
            0.01,
            GREEN,
        );

        root_ui().window(321, vec2(10.0, 10.0), vec2(500.0, 60.0), |ui| {
            ui.slider(123, "1/dx", 1.0..100.0, &mut delta_inv);
            ui.label(None, &label_str)
        });
        delta_inv = delta_inv.floor();
        //root_ui().slider(123,"DX",0.0..0.5,&mut delta);
        let delta = 1.0 / delta_inv;
        let pt_size = delta.min(0.02);
        let w_x = world_pt.x;
        let w_y = world_pt.y;
        draw_circle(w_x, w_y, 0.001, RED);
        let max_x = cam
            .screen_to_world(Vec2::new(screen_width(), screen_height()))
            .x;
        
        let min_x = cam.screen_to_world(Vec2::ZERO).x;
        let forward_stream = successors(Some((w_x, w_y)), move |(x, y)| {
            Some(forward_euler_step(*x, *y, |_x, y| y, delta))
        });
        let forward_points: Vec<(f32, f32)> =
            forward_stream.take_while(|(x, _)| *x <= max_x).collect();

        for (wx, wy) in forward_points {
            draw_circle(wx, wy, pt_size, BLUE);
            
            if f32::abs(wx - 1.0) < 0.00001 {
                label_str = (-1.0 * wy).to_string();
            }
        }
        let backwards_stream = successors(Some((w_x, w_y)), move |(x, y)| {
            Some(forward_euler_step(*x, *y, |_x, y| y, -1.0 * delta))
        });
        let backwards_points: Vec<(f32, f32)> =
            backwards_stream.take_while(|(x, _)| *x >= min_x).collect();

        for (wx, wy) in backwards_points {
            draw_circle(wx, wy, pt_size, BLUE);
            
        }

        let mut xval = min_x;
        let mut yval = -1.0*xval.exp();
        while xval < max_x {
            let new_xval = xval + 0.001;
            let new_yval = -1.0*new_xval.exp();
            draw_line(xval, yval, new_xval, new_yval, pt_size/2.0, RED);
            xval = new_xval;
            yval = new_yval;
        }

        next_frame().await
    }
}
