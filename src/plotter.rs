//! **plotter** provides the `Plotter` object which handles plot creation and lifecycles
//!
//! Each plot runs asynchronously in a background thread. A `Plotter` creates and tracks these background threads.
//!
//! For now, `Plotter::plot2d` is the only supported plotting function. It takes a `PlotBuilder2D` containing all needed information.
//!
//! The `Plotter::join` function allows the thread that owns the `Plotter` to wait until the user has closed all open plot windows before continuing.

use std::thread;
use plotbuilder::PlotBuilder2D;
use plot::Plot;

pub struct Plotter {
    plots: Vec<thread::JoinHandle<()>>,
}

impl Plotter {
    /// `new` creates a new `Plotter` object to manage asynchronous plots
    pub fn new() -> Plotter {
        Plotter { plots: Vec::new() }
    }

    /// `plot2d` is currently the only supported plotting function. It takes a `PlotBuilder2D` containing all needed information.
    pub fn plot2d(&mut self, plotbuilder: PlotBuilder2D) {
        self.plots.push(thread::spawn(move || {
            Plot::new2d(plotbuilder);
        }));
    }

    /// The `join` function allows the thread that owns the `Plotter` to wait until the user has closed all open plot windows before continuing.
    pub fn join(self) {
        for t in self.plots {
            let _ = t.join();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use plotbuilder::*;
    use util::*;

    #[test]
    fn plot2d_test() {

        let x = linspace(0, 10, 100);
        let y = (&x).iter().map(|x| x.sin()).collect();
        let xy = zip2(&x, &y);

        let mut pb1 = PlotBuilder2D::new();
        pb1.add_simple_xy(xy);
        let mut plt = Plotter::new();
        plt.plot2d(pb1);
        plt.join();
    }
}
