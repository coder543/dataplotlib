extern crate dataplotlib;
use dataplotlib::util::{linspace, zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;

fn main() {
    let x = linspace(0, 10, 100);

    let y_sin = x.iter().map(|x| x.sin()).collect();
    let xy_sin = zip2(&x, &y_sin);

    let xy_lin = zip2(&x, &x);

    // Creates a new plot builder
    let mut pb = PlotBuilder2D::new();

    // Adds the sin plot and the cos plot
    pb.add_simple_xy(xy_sin);
    pb.add_simple_xy(xy_lin);

    let mut plt = Plotter::new();
    plt.plot2d(pb);
    plt.join();
}
