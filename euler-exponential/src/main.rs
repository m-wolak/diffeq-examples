use std::iter::successors;

use macroquad::prelude::*;
use macroquad::ui::*;

// This is the function that matters to us: Euler's method for approximating a differential equation
// 
fn forward_euler_step(last_x: f32, last_y: f32, f: fn(f32, f32) -> f32, delta: f32) -> (f32, f32) {
    let next_y = last_y + delta * f(last_x, last_y); // new y is old y plus delta times the derivative
    let next_x = last_x + delta;                    // new x is old x plus delta
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

#[macroquad::main("Test-o-matic")]
async fn main() {
    let mut delta_inv: f32 = 10.0; //default to delta_x = 1/10


    // Set up the camera
    let w = screen_width();
    let h = screen_height();
    let mut ar = (w as f32) / (h as f32);
    let cam = Camera2D::from_display_rect(Rect {
        x: -1.5 * ar,
        y: -3.0,
        w: 3.0 * ar,
        h: 3.0,
    });
    set_camera(&cam);
    
    // default label for e display
    let mut label_str = "".to_string();
    
    loop {
        
        let w = screen_width();
        let h = screen_height();
        let mut euler_e = 0.0;
        let mut rk4_e = 0.0;
        
        let new_ar = (w as f32) / (h as f32);  // check if we need to update the camera
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

        // This is the starting point for the diffeq. It's upside down because
        // macroquad has increasing y -> down
        let world_pt = vec2(0.0, -1.0);


        // Draw the x and y axes
        draw_line(0.0, 0.0, 0.0, -10.0, 0.01, BLACK);
        draw_line(-10.0, 0.0, 10.0, 0.0, 0.01, BLACK);

        // Draw in lines at x = 1 and y = e
        draw_line(1.0, 0.0, 1.0, -10.0, 0.01, GREEN);
        draw_line(
            -10.0,
            -1.0 * 1.0_f32.exp(),
            10.0,
            -1.0 * 1.0_f32.exp(),
            0.01,
            GREEN,
        );

        // set up the slider to change delta_x
        root_ui().window(321, vec2(10.0, 10.0), vec2(500.0, 60.0), |ui| {
            ui.slider(123, "1/delta_x", 1.0..100.0, &mut delta_inv);
            ui.label(None, &label_str)
        });
        delta_inv = delta_inv.floor(); // we want an integer number of steps per unit time
        
        let delta = 1.0 / delta_inv; // delta is delta_x
        let pt_size = delta.min(0.02);
        let w_x = world_pt.x;
        let w_y = world_pt.y;
        draw_circle(w_x, w_y, pt_size, RED); // draw a red dot at the starting time


        // when to stop calculating points
        let max_x = cam
            .screen_to_world(Vec2::new(screen_width(), screen_height()))
            .x;
        
        let min_x = cam.screen_to_world(Vec2::ZERO).x;

        // Here is where we calculate the diffeq using Euler's method, forward from (0,1)
        let forward_stream = successors(Some((w_x, w_y)), move |(x, y)| {
            Some(forward_euler_step(*x, *y, |_x, y| y, delta))
        });
        let forward_points: Vec<(f32, f32)> =
            forward_stream.take_while(|(x, _)| *x <= max_x).collect();

        let rk4_stream = successors(Some((w_x, w_y)), move |(x, y)| {
            Some(rk4_step(*x, *y, |_x, y| y, delta))
        });

        let rk4_points : Vec<(f32, f32)> =
            rk4_stream.take_while(|(x, _)| *x <= max_x).collect();

        // draw in the forward points
        for (wx, wy) in forward_points {
            draw_circle(wx, wy, pt_size, BLUE);
            
            if f32::abs(wx - 1.0) < 0.00001 {
                euler_e = -1.0*wy;
            }
        }

        for (wx, wy) in rk4_points {
            draw_circle(wx, wy, pt_size, PURPLE);
            if f32::abs(wx - 1.0) < 0.00001 {
                rk4_e = -1.0*wy;
            }
            
        }

        label_str = format!("Euler's e approx: {}, rk4 e approx: {}", euler_e, rk4_e);
        // Calculate the diffeq backwards from (0,1). With Euler's method, it's as easy as picking a negative delta_x!
        let backwards_stream = successors(Some((w_x, w_y)), move |(x, y)| {
            Some(forward_euler_step(*x, *y, |_x, y| y, -1.0 * delta))
        });
        let backwards_points: Vec<(f32, f32)> =
            backwards_stream.take_while(|(x, _)| *x >= min_x).collect();

        let backwards_rk4_stream = successors(Some((w_x, w_y)), move |(x, y)| {
            Some(rk4_step(*x, *y, |_x, y| y, -1.0*delta))
        });

        let backwards_rk4_points : Vec<(f32, f32)> = backwards_rk4_stream.take_while(|(x, _)| *x >= min_x).collect();

        // draw in backward points
        for (wx, wy) in backwards_points {
            draw_circle(wx, wy, pt_size, BLUE);
            
        }
        for (wx, wy) in backwards_rk4_points {
            draw_circle(wx, wy, pt_size, PURPLE);
            
        }

        // This part just draws in the true graph of y = e^x
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
