use piston_window;
use piston_window::*;
use std::thread;
use plotbuilder::*;
// use sdl2_window::Sdl2Window;


pub struct Plot {
    window: PistonWindow, // <Sdl2Window>
    plotdata: PlotBuilder2D,
}

// pt: a point on a 1 dimensional line segment
// min: the closest point to render on the line segment
// max: the farthest point to render on the line segment
// length: the length of the 1 dimensional window space
fn point2plot(pt: f64, min: f64, max: f64, length: f64) -> f64 {
    ((pt - min) / max) * length
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

fn get_min(user_max: Option<f64>, values: &Vec<f64>) -> f64 {
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

impl Plot {
    pub fn new2d(plotbuilder: PlotBuilder2D) {
        let mut plt = Plot {
            window: WindowSettings::new("2D plot", [720, 360])
                .opengl(piston_window::OpenGL::V3_2)
                .samples(4)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            plotdata: plotbuilder,
        };

        plt.window.set_ups(60);

        if let PlotVals2D::Xy(ref xy) = plt.plotdata.pvs[0] {

            if xy.len() <= 1 {
                return;
            }

            let mut xs: Vec<f64> = Vec::with_capacity(xy.len());
            let mut ys: Vec<f64> = Vec::with_capacity(xy.len());

            for &(x, y) in xy {
                xs.push(x);
                ys.push(y);
            }

            let x_max = get_max(plt.plotdata.max_x, &xs);
            let y_max = get_max(plt.plotdata.max_y, &ys);

            let x_min = get_min(plt.plotdata.min_x, &xs);
            let y_min = get_min(plt.plotdata.min_y, &ys);


            // Poll events from the window.
            while let Some(event) = plt.window.next() {
                let w = plt.window.size().width as f64;
                let h = plt.window.size().height as f64;

                let xt: Vec<f64> = xs.iter().map(|x| point2plot(*x, x_min, x_max, w)).collect();
                let yt: Vec<f64> = ys.iter().map(|y| point2plot(*y, y_min, y_max, h)).collect();

                plt.window.draw_2d(&event, |c, g| {
                    let color = xs[0] as f32;
                    clear([color, color, color, 1.0], g);

                    for i in 0..xy.len() - 1 {
                        let (xa, ya) = (xt[i + 0], yt[i + 0]);
                        let (xb, yb) = (xt[i + 1], yt[i + 1]);
                        line([0.0, 1.0, 0.0, 1.0], 3.0, [xa, ya, xb, yb], c.transform, g);
                        // rectangle([1.0, 0.0, 0.0, 1.0], // red
                        //           [0.0, 0.0, 100.0, 100.0], // rectangle
                        //           c.transform,
                        //           g);
                    }
                });
            }

        }

    }
}
