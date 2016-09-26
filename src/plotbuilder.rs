//! **plotbuilder** provides the `struct`s that organize the plot data, plus some helper functions
//!
//! It is recommended to use `PlotBuilder2D::simple_xy` rather than manually instantiating the `PlotBuilder2D` struct,
//! but feel free to approach this in the most ergonomic fashion for you.

use std::marker::Sync;

pub type PlotFn = &'static (Fn(f64) -> f64 + Sync);
pub type AnimFn = &'static (Fn(f64, f64) -> f64 + Sync);

/// `PlotVals2D` provides all of the value data for an individual plot
/// Note: Only `Xy()` does anything at the moment
#[derive(Clone)]
pub enum PlotVals2D {
    /// A simple x-y value line plot
    Xy(Vec<(f64, f64)>),

    /// A simple x-y value line plot... now in a color of your choice!
    XyColor([f32; 4], Vec<(f64, f64)>),

    /// A function that produces a y value for a given x value
    Fun(PlotFn),

    /// with color!
    FunColor([f32; 4], PlotFn),

    /// A function that produces a y value given both an x value and a time value between 0 and 1
    AnimFun(AnimFn),

    /// Probably can't figure out what this does
    AnimFunColor([f32; 4], AnimFn),

    /// A set of height values needed to produce a bar chart
    Bars(Vec<f64>),

    /// Adds a user-selected color to all bars. We probably want to be able to choose per-bar colors.
    BarsColor([f32; 4], Vec<f64>),
}

/// `PlotBuilder2D` contains all of the necessary information to create a
/// series of stacked 2 dimensional plots. For the moment, only provide one
/// `PlotVals2D`, otherwise things will probably go poorly.
#[derive(Clone)]
pub struct PlotBuilder2D {
    /// **pvs** contains the **P**lot **V** alue **s**
    pub pvs: Vec<PlotVals2D>,

    /// **min_x** optionally defines the lower x bound. If `None`, it will be auto determined.
    pub min_x: Option<f64>,

    /// **max_x** optionally defines the upper x bound. If `None`, it will be auto determined.
    pub max_x: Option<f64>,

    /// **min_y** optionally defines the lower y bound. If `None`, it will be auto determined.
    pub min_y: Option<f64>,

    /// **max_y** optionally defines the upper y bound. If `None`, it will be auto determined.
    pub max_y: Option<f64>,

    /// A string to label the x-axis. (not implemented)
    pub x_label: Option<String>,

    /// A string to label the y-axis. (not implemented)
    pub y_label: Option<String>,

    /// A string to label the chart. (not implemented)
    pub title: Option<String>,

    /// Whether or not to draw the y-axis. (not implemented)
    pub y_axis: bool,

    /// Whether or not to draw the gridlines on the y-axis. (not implemented)
    pub y_gridlines: bool,

    /// Whether or not to draw the x-axis. (not implemented)
    pub x_axis: bool,

    /// Whether or not to draw the gridlines on the x-axis. (not implemented)
    pub x_gridlines: bool,

    /// The font file to use for any text. (not implemented)
    pub font_path: String,
}

const DEFAULT_FONT: &'static str = "/usr/share/fonts/truetype/freefont/FreeSans.ttf";

impl PlotBuilder2D {
    /// `simply_xy` reduces boilerplate by generating some basic defaults for the `PlotBuilder2D` struct.
    /// Once the struct is returned, it's easy enough to make adjustments.
    pub fn simple_xy(xy: Vec<(f64, f64)>) -> PlotBuilder2D {
        PlotBuilder2D {
            pvs: vec![PlotVals2D::Xy(xy)],
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            x_label: None,
            y_label: None,
            title: None,
            y_axis: true,
            y_gridlines: true,
            x_axis: true,
            x_gridlines: true,
            font_path: DEFAULT_FONT.to_string(),
        }
    }

    /// A slight variation of `simply_xy` which gives the graph a color.
    /// Once the struct is returned, it's easy enough to make adjustments.
    pub fn simple_xy_colored(xy: Vec<(f64, f64)>, color: [f32; 4]) -> PlotBuilder2D {
        PlotBuilder2D {
            pvs: vec![PlotVals2D::XyColor(color, xy)],
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            x_label: None,
            y_label: None,
            title: None,
            y_axis: true,
            y_gridlines: true,
            x_axis: true,
            x_gridlines: true,
            font_path: DEFAULT_FONT.to_string(),
        }
    }

    /// `simple_fun` works very similarly to `simple_xy`, except that it should not be used right now.
    pub fn simple_fun(plotfn: PlotFn) -> PlotBuilder2D {
        PlotBuilder2D {
            pvs: vec![PlotVals2D::Fun(plotfn)],
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            x_label: None,
            y_label: None,
            title: None,
            y_axis: true,
            y_gridlines: true,
            x_axis: true,
            x_gridlines: true,
            font_path: DEFAULT_FONT.to_string(),
        }
    }
}