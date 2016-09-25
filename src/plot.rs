use piston_window;
use piston_window::*;
use std::cmp::min;
use plotbuilder::*;
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

fn draw_borders(bordercol: [f32; 4],
                bgcol: [f32; 4],
                space: f64,
                m: f64,
                transform: [[f64; 3]; 2],
                g: &mut piston_window::G2d) {
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

fn draw_xy(plotdata: PlotBuilder2D, xy: &Vec<(f64, f64)>, window: &mut PistonWindow) {

    let bordercol = [0.95, 0.95, 0.95, 1.0];
    let bgcol = [1.0, 1.0, 1.0, 1.0];
    let margin = 0.05;
    let invmargin = 1.0 - 2.0 * margin;

    if xy.len() <= 1 {
        return;
    }

    let mut xs: Vec<f64> = Vec::with_capacity(xy.len());
    let mut ys: Vec<f64> = Vec::with_capacity(xy.len());

    for &(x, y) in xy {
        xs.push(x);
        ys.push(y);
    }

    let x_max = get_max(plotdata.max_x, &xs);
    let y_max = get_max(plotdata.max_y, &ys);

    let x_min = get_min(plotdata.min_x, &xs);
    let y_min = get_min(plotdata.min_y, &ys);


    // Poll events from the window.
    while let Some(event) = window.next() {
        let w = window.size().width;
        let h = window.size().height;

        let m = min(w, h) as f64;
        let space = m * margin;
        let m = m * invmargin;

        let xt: Vec<f64> = xs.iter().map(|x| point2plot(*x, x_min, x_max, m, space)).collect();
        let yt: Vec<f64> = ys.iter()
            .map(|y| (2.0 * space + m) - point2plot(*y, y_min, y_max, m, space))
            .collect();

        window.draw_2d(&event, |c, g| {

            draw_borders(bordercol, bgcol, space, m, c.transform, g);

            for i in 0..xy.len() - 1 {
                let (xa, ya) = (xt[i + 0], yt[i + 0]);
                let (xb, yb) = (xt[i + 1], yt[i + 1]);
                line([0.0, 1.0, 0.0, 1.0], 1.0, [xa, ya, xb, yb], c.transform, g);

            }
        });
    }
}

impl Plot {
    pub fn new2d(plotdata: PlotBuilder2D) {
        let mut window: PistonWindow = WindowSettings::new("2D plot", [720, 720])
            .opengl(piston_window::OpenGL::V3_2)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut plotdata = plotdata;

        // let mut ui = conrod::UiBuilder::new().build();
        // ui.fonts.insert_from_file(plotdata.font_path).unwrap();

        // // Create a texture to use for efficiently caching text on the GPU.
        // let text_texture_cache =
        //     conrod::backend::piston_window::GlyphCache::new(&mut window, 720, 720);

        window.set_ups(60);

        let mut pvs = Vec::new();

        std::mem::swap(&mut plotdata.pvs, &mut pvs);

        for pv in pvs.drain(..) {
            match pv {
                PlotVals2D::Xy(ref xy) => draw_xy(plotdata.clone(), xy, &mut window),
                _ => (),
            }
        }

    }
}
