//! **util** just provides a couple of utility functions to make life easier.

use std::cmp::min;

/// `linspace` allows eases the creation a `Vec` of values to use as the x values on a plot.
/// Once that is done, it is straightforward to map some function over that `Vec` to get the y values.
pub fn linspace<T>(start: T, end: T, steps: u64) -> Vec<f64>
    where T: Into<f64>
{
    let start: f64 = start.into();
    let end: f64 = end.into();
    let mut v = Vec::new();
    let stepsize = (end - start) / (steps as f64);
    for step in 0..steps {
        v.push(start + step as f64 * stepsize);
    }
    return v;
}

/// `zip2` will combine two `Vec<T>` into a single `Vec<(T, T)>` with a length equal to the length of the shorter input `Vec`.
/// Rust has a built in `zip` function, but it has a signature that yields `Vec<(&T, &T)>` which is undesirable.
pub fn zip2<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<(T, T)>
    where T: Copy
{
    let shorter = min(a.len(), b.len());
    let mut ret = Vec::new();
    for i in 0..shorter {
        ret.push((a[i], b[i]));
    }
    return ret;
}
