//! dataplotlib is a hassle-free library for plotting data
//!
//! # Example of how easy it is to use:
//!
//! ```
//! extern crate dataplotlib;
//! use dataplotlib::util::{linspace, zip2};
//! use dataplotlib::plotbuilder::PlotBuilder2D;
//! use dataplotlib::plotter::Plotter;
//!
//! fn main() {
//!     let x = linspace(0, 10, 100);
//!
//!     let y_sin = x.iter().map(|x| x.sin()).collect();
//!     let xy_sin = zip2(&x, &y_sin);
//!
//!     let xy_lin = zip2(&x, &x);
//!
//!     // Creates a new plot builder
//!     let mut pb = PlotBuilder2D::new();
//!
//!     // Adds the sin plot and the linear plot with custom colors
//!     pb.add_color_xy(xy_sin, [1.0, 0.0, 0.0, 1.0]);
//!     pb.add_color_xy(xy_lin, [0.0, 0.0, 1.0, 1.0]);
//!
//!     let mut plt = Plotter::new();
//!     plt.plot2d(pb);
//!     plt.join();
//! }
//! ```

#[macro_use]
extern crate piston_window;
// extern crate sdl2_window;

mod plot;
pub mod plotter;
pub mod plotbuilder;
pub mod util;
