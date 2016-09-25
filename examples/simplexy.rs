extern crate dataplotlib;
use dataplotlib::util::{linspace, zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;

fn main() {
    let x = linspace(0, 10, 100);
    let y = (&x).iter().map(|x| x.sin()).collect();
    let xy = zip2(&x, &y);

    let pb = PlotBuilder2D::simple_xy(xy);
    let mut plt = Plotter::new();
    plt.plot2d(pb);
    plt.join();
}