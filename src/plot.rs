//! **plot** is the backend that actually renders the plots.
//!
//! Users of **dataplotlib** should not need to access **plot**.

use std::time::Duration;
use std::{mem, thread, f64};

use plotbuilder::*;

use draw::{Drawable, Event, Range};

pub struct Plot {}

fn get_max(user_max: Option<f64>, values: &Vec<f64>) -> f64 {
    if let Some(max) = user_max {
        max
    } else {
        let mut max = *values.first().unwrap();
        for val in values {
            if *val > max {
                max = *val;
            }
        }
        max
    }
}

fn get_min(user_min: Option<f64>, values: &Vec<f64>) -> f64 {
    if let Some(min) = user_min {
        min
    } else {
        let mut min = *values.first().unwrap();
        for val in values {
            if *val < min {
                min = *val;
            }
        }
        min
    }
}

fn f32_4_to_color(col: [f32; 4]) -> [u8; 4] {
    [
        (col[0] * 255f32) as u8,
        (col[2] * 255f32) as u8,
        (col[1] * 255f32) as u8,
        (col[3] * 255f32) as u8,
    ]
}

fn draw_borders(bordercol: [u8; 4], bgcol: [u8; 4], space: (f64, f64), m: (f64, f64), renderer: &mut Drawable) {
    renderer.set_color(bordercol);
    renderer.clear();

    renderer.set_color(bgcol);
    renderer.rectangle(space, m);

    renderer.set_color([0, 0, 255, 255]);
    renderer.unfilled_rectangle(space, m);
}

fn set_xy(xy: &Vec<(f64, f64)>, x_vector: &mut Vec<Vec<f64>>, y_vector: &mut Vec<Vec<f64>>) {
    x_vector.push(Vec::new());
    y_vector.push(Vec::new());

    let last_index = x_vector.len() - 1;

    for &(x, y) in xy {
        x_vector[last_index].push(x);
        y_vector[last_index].push(y);
    }
}

fn simple_clamp(v: f64, r: Range) -> f64 {
    if v > r.max {
        r.max
    } else if v < r.min {
        r.min
    } else {
        v
    }
}

fn clamp(mut a: (f64, f64), mut b: (f64, f64), w: Range, h: Range) -> ((f64, f64), (f64, f64)) {
    let slope = (b.1 - a.1) / (b.0 - a.0);
    let offset_b = a.1 - slope * a.0;
    println!(
        "eq = {}x + {}\na: {:?}, b: {:?}\nw: {:?}, h: {:?}",
        slope,
        offset_b,
        a,
        b,
        w,
        h
    );
    if a.0 < w.min && b.0 > w.min {
        a.0 = w.min;
        a.1 = slope * a.0 + offset_b;
        clamp(a, b, w, h)
    } else if a.1 < h.min && b.1 > h.min {
        let mut x_b = a.0 / offset_b;
        if !f64::is_finite(x_b) {
            x_b = 0.0;
        }
        a.1 = h.min;
        a.0 = a.1 / slope + x_b;
        clamp(a, b, w, h)
    } else if b.0 < w.min && a.0 > w.min {
        b.0 = w.min;
        b.1 = slope * b.0 + offset_b;
        clamp(a, b, w, h)
    } else if b.1 < h.min && a.1 > h.min {
        let mut x_b = b.0 / offset_b;
        if !f64::is_finite(x_b) {
            x_b = 0.0;
        }
        b.1 = h.min;
        b.0 = b.1 / slope + x_b;
        clamp(a, b, w, h)
    } else if a.0 > w.max && b.0 < w.max {
        a.0 = w.max;
        a.1 = slope * a.0 + offset_b;
        clamp(a, b, w, h)
    } else if a.1 > h.max && b.1 < h.max {
        let mut x_b = a.0 / offset_b;
        if !f64::is_finite(x_b) {
            x_b = 0.0;
        }
        a.1 = h.max;
        a.0 = a.1 / slope + x_b;
        clamp(a, b, w, h)
    } else if b.0 > w.max && a.0 < w.max {
        b.0 = w.max;
        b.1 = slope * b.0 + offset_b;
        clamp(a, b, w, h)
    } else if b.1 > h.max && a.1 < h.max {
        let mut x_b = b.0 / offset_b;
        if !f64::is_finite(x_b) {
            x_b = 0.0;
        }
        b.1 = h.max;
        b.0 = b.1 / slope + x_b;
        clamp(a, b, w, h)
    } else {
        println!("done\n");
        a = (simple_clamp(a.0, w), simple_clamp(a.1, h));
        b = (simple_clamp(b.0, w), simple_clamp(b.1, h));
        (a, b)
    }
}

fn draw_plots(renderer: &mut Drawable, xs: &Vec<Vec<f64>>, ys: &Vec<Vec<f64>>, colors: &Vec<[f32; 4]>, plot_bounds: [f64; 4]) {
    let bordercol = f32_4_to_color([0.95, 0.95, 0.95, 1.0]);
    let bgcol = f32_4_to_color([1.0, 1.0, 1.0, 1.0]);
    let margin = 0.05;

    let w = Range {
        min: plot_bounds[2],
        max: plot_bounds[0],
    };

    let h = Range {
        min: plot_bounds[3],
        max: plot_bounds[1],
    };

    renderer.set_view(w, h);

    let update_frame = |renderer: &mut Drawable| {
        let (w, h) = renderer.get_view();

        // calculate margins around plot
        let w_marg = w.size() * margin;
        let h_marg = h.size() * margin;

        // set up a "fake" plot view that has extra margins
        let w_fake = Range {
            min: w.min - w_marg,
            max: w.max + w_marg,
        };
        let h_fake = Range {
            min: h.min - h_marg,
            max: h.max + h_marg,
        };
        renderer.set_view(w_fake, h_fake);

        // the borders are just the edges of the real view
        let border_min = (w.min, h.min);
        let border_max = (w.max, h.max);

        draw_borders(bordercol, bgcol, border_min, border_max, renderer);

        for i in 0..colors.len() {
            let color = colors[i];
            let color_rgba = f32_4_to_color(color);
            renderer.set_color(color_rgba);

            let yt = &ys[i];
            let xt = &xs[i];

            // The number of points
            let len = xs[i].len();
            for j in 0..len - 1 {
                let a = (xt[j + 0], yt[j + 0]);
                let b = (xt[j + 1], yt[j + 1]);
                let ((xa, ya), (xb, yb)) = clamp(a, b, w, h);
                let x_invalid = (xa <= w.min && xb <= w.min) || (xa >= w.max && xb >= w.max);
                let y_invalid = (ya <= h.min && yb <= h.min) || (ya >= h.max && yb >= h.max);

                if !(x_invalid || y_invalid) {
                    renderer.thick_line((xa, ya), (xb, yb), 2);
                }
            }
        }

        // reset the view to the real view
        renderer.set_view(w, h);
    };


    update_frame(renderer);

    'main: loop {
        let mut update = false;
        for event in renderer.get_events() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown(keycode) => {
                    if keycode == 1 {
                        //Keycode::Escape {
                        break 'main;
                    }
                }
                Event::MouseScroll(_x, y) => {
                    let multiplier = (y as f64) / 10.0;
                    let (w, h) = renderer.get_view();
                    let w_offset = w.size() * multiplier;
                    let new_w = Range {
                        min: w.min - w_offset,
                        max: w.max + w_offset,
                    };
                    let h_offset = h.size() * multiplier;
                    let new_h = Range {
                        min: h.min - h_offset,
                        max: h.max + h_offset,
                    };
                    renderer.set_view(new_w, new_h);
                    update = true;
                }
                Event::Resize(_, _) => {
                    update = true;
                }
                _ => {}
            }
        }

        if update {
            update_frame(renderer);
        }

        thread::sleep(Duration::from_millis(16));
    }
}

fn get_plot_bounds(plot_builder: &PlotBuilder2D, xs: &Vec<Vec<f64>>, ys: &Vec<Vec<f64>>) -> [f64; 4] {

    let mut max_xs: Vec<f64> = Vec::new();
    let mut max_ys: Vec<f64> = Vec::new();
    let mut min_xs: Vec<f64> = Vec::new();
    let mut min_ys: Vec<f64> = Vec::new();

    // Get the plot extremities
    for i in 0..xs.len() {
        max_xs.push(get_max(plot_builder.max_x, &xs[i]));
        max_ys.push(get_max(plot_builder.max_y, &ys[i]));

        min_xs.push(get_min(plot_builder.min_x, &xs[i]));
        min_ys.push(get_min(plot_builder.min_y, &ys[i]));
    }

    let plot_bounds: [f64; 4] = [
        // Apply the plot extremities to the global extremities
        max_xs.iter().cloned().fold(0. / 0., f64::max),
        max_ys.iter().cloned().fold(0. / 0., f64::max),
        min_xs.iter().cloned().fold(0. / 0., f64::min),
        min_ys.iter().cloned().fold(0. / 0., f64::min),
    ];

    plot_bounds
}

impl Plot {
    pub fn new2d(mut plot_builder: PlotBuilder2D, mut renderer: Box<Drawable>) {
        let mut pvs = Vec::new();

        mem::swap(&mut plot_builder.pvs, &mut pvs);

        let mut colors: Vec<[f32; 4]> = Vec::new();
        let mut x_points: Vec<Vec<f64>> = Vec::new();
        let mut y_points: Vec<Vec<f64>> = Vec::new();

        for pv in pvs.drain(..) {
            match pv {
                PlotVals2D::XyColor(ref col, ref xy) => {
                    set_xy(xy, &mut x_points, &mut y_points);
                    colors.push(col.clone());
                }
                _ => (),
            }
        }

        // [MAX_X, MAX_Y, MIN_X, MIN_Y]
        let plot_bounds: [f64; 4] = get_plot_bounds(&plot_builder, &x_points, &y_points);
        draw_plots(&mut *renderer, &x_points, &y_points, &colors, plot_bounds);
    }
}
