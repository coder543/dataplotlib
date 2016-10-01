//! **plot** is the backend that actually renders the plots.
//!
//! Users of **dataplotlib** should not need to access **plot**.

use piston_window;
use piston_window::*;
use std::cmp::min;
use plotbuilder::*;
use std::f64;
use std;
// use sdl2_window::Sdl2Window;


pub struct Plot {
}

// pt: a point on a 1 dimensional line segment
// min: the closest point to render on the line segment
// max: the farthest point to render on the line segment
// length: the length of the 1 dimensional window space
// space: the offset from the beginning of the line segment
fn point2plot(pt: f64, min: f64, max: f64, length: f64, space: f64) -> f64 {
    ((pt - min) / (max - min)) * length + space
}

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

fn draw_borders(bordercol: [f32; 4], bgcol: [f32; 4], space: f64, m: f64, transform: [[f64; 3]; 2], g: &mut piston_window::G2d) {
    clear(bordercol, g);
    rectangle([0.0, 0.0, 1.0, 1.0],
              [space - 2.0, space - 2.0, m + 4.0, m + 4.0], // rectangle
              transform,
              g);
    rectangle(bgcol,
              [space - 1.0, space - 1.0, m + 2.0, m + 2.0], // rectangle
              transform,
              g);
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

fn draw_plots(window: &mut PistonWindow, xs: &Vec<Vec<f64>>, ys: &Vec<Vec<f64>>, colors: &Vec<[f32; 4]>, plot_bounds: [f64; 4]) {
    let bordercol = [0.95, 0.95, 0.95, 1.0];
    let bgcol = [1.0, 1.0, 1.0, 1.0];
    let margin = 0.05;
    let invmargin = 1.0 - 2.0 * margin;

    let w = window.size().width;
    let h = window.size().height;

    let m = min(w, h) as f64;
    let space = m * margin;
    let m = m * invmargin;

    let x_max = plot_bounds[0];
    let y_max = plot_bounds[1];
    let x_min = plot_bounds[2];
    let y_min = plot_bounds[3];

    // Poll events from the window.
    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            draw_borders(bordercol, bgcol, space, m, c.transform, g);
        });

        for i in 0..colors.len() {
            let color = colors[i];
            let xt: Vec<f64> = xs[i].iter().map(|x| point2plot(*x, x_min, x_max, m, space)).collect();
            let yt: Vec<f64> = ys[i]
                .iter()
                .map(|y| (2.0 * space + m) - point2plot(*y, y_min, y_max, m, space))
                .collect();

            window.draw_2d(&event, |c, g| {
                // The number of points
                let len = xs[i].len();
                for i in 0..len - 1 {
                    let (xa, ya) = (xt[i + 0], yt[i + 0]);
                    let (xb, yb) = (xt[i + 1], yt[i + 1]);
                    line([color[0], color[1], color[2], color[3]],
                         1.0,
                         [xa, ya, xb, yb],
                         c.transform,
                         g);
                }
            });
        }
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

    let plot_bounds: [f64; 4] = [// Apply the plot extremities to the global extremities
                                 max_xs.iter().cloned().fold(0. / 0., f64::max),
                                 max_ys.iter().cloned().fold(0. / 0., f64::max),
                                 min_xs.iter().cloned().fold(0. / 0., f64::min),
                                 min_ys.iter().cloned().fold(0. / 0., f64::min)];

    plot_bounds
}

impl Plot {
    pub fn new2d(plot_builder: PlotBuilder2D) {
        let mut window: PistonWindow = WindowSettings::new("2D plot", [720, 720])
            .opengl(piston_window::OpenGL::V3_2)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut plot_builder = plot_builder;

        // let mut ui = conrod::UiBuilder::new().build();
        // ui.fonts.insert_from_file(plot_builder.font_path).unwrap();

        // // Create a texture to use for efficiently caching text on the GPU.
        // let text_texture_cache =
        //     conrod::backend::piston_window::GlyphCache::new(&mut window, 720, 720);

        window.set_ups(60);

        let mut pvs = Vec::new();

        std::mem::swap(&mut plot_builder.pvs, &mut pvs);

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
        draw_plots(&mut window, &x_points, &y_points, &colors, plot_bounds);
    }
}
