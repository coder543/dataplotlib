#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Quit,
    Resize(f64, f64),
    KeyDown(i32),
    KeyUp(i32),
    MouseDown(MouseButton, f64, f64),
    MouseUp(MouseButton, f64, f64),
    MouseMove(MouseButton, f64, f64),
    MouseScroll(i32, i32),
}

#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, pt: f64) -> bool {
        pt <= self.max && pt >= self.min
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Range2d(pub Range, pub Range);

impl Range2d {
    pub fn contains(&self, pt: (f64, f64)) -> bool {
        self.0.contains(pt.0) && self.1.contains(pt.1)
    }
}

impl From<(Range, Range)> for Range2d {
    fn from((a, b): (Range, Range)) -> Self {
        Range2d(a, b)
    }
}

pub trait Drawable: Send {
    /// Sets the visible range of worldspace
    fn set_view(&mut self, view: Range2d);

    /// Gets the visible range of worldspace
    fn get_view(&self) -> Range2d;

    /// Set color for various drawing actions
    fn set_color(&mut self, color: [u8; 4]);

    /// Clears the output surface
    fn clear(&mut self);

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn line(&mut self, a: (f64, f64), b: (f64, f64));

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn thick_line(&mut self, a: (f64, f64), b: (f64, f64), thickness: u16);

    /// Draws a rectangle bounded by two corners
    fn rectangle(&mut self, a: (f64, f64), b: (f64, f64));

    /// Draws a rectangle bounded by two corners
    fn unfilled_rectangle(&mut self, a: (f64, f64), b: (f64, f64));

    /// Presents the previously drawn buffer
    fn present(&mut self);

    /// Returns the next pending events
    fn get_events(&mut self) -> Vec<Event>;

    /// Asks that the Drawable stop any tasks and cleanup
    fn close(&mut self) {} // provide empty default impl
}

pub fn point2window(pt: f64, view: Range, window: Range, invert: bool) -> f64 {
    let moved_pt = if invert { view.max - pt } else { pt - view.min };

    (moved_pt / view.size()) * (window.size() + window.min)
}