use std::thread;
use plotbuilder::{PlotBuilder2D, PlotVals2D};
use plot::Plot;

pub struct Plotter {
    plots: Vec<thread::JoinHandle<()>>,
}

impl Plotter {
    pub fn new() -> Plotter {
        Plotter { plots: Vec::new() }
    }

    pub fn plot2d(&mut self, plotbuilder: PlotBuilder2D) {
        self.plots.push(thread::spawn(move || {
            Plot::new2d(plotbuilder);
        }));
    }

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

    #[test]
    fn plot2d_test() {

        let mut x = Vec::new();
        for v in 0..10 {
            x.push(v / 0.3);
        }

        let pb1 = PlotBuilder2D::simple_xy(vec![(0.75, 1.0), (10.0, 10.0)]);
        let pb2 = PlotBuilder2D::simple_xy(vec![(0.3, 1.0), (25.0, 180.0)]);
        let mut plt = Plotter::new();
        plt.plot2d(pb1);
        plt.plot2d(pb2);
        plt.join();
    }
}