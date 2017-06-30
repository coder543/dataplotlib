#![allow(dead_code)]

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum Event {
    Quit,
    Resize(f64, f64),
    KeyDown(i32),
    KeyUp(i32),
    MouseDown(Option<MouseButton>, f64, f64),
    MouseUp(Option<MouseButton>, f64, f64),
    MouseMove(Option<MouseButton>, f64, f64),
}

#[derive(Copy, Clone)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

pub trait Drawable {
    /// Sets the visible range of worldspace
    fn set_view(&mut self, x: Range, y: Range);

    /// Set color for various drawing actions
    fn set_color(&mut self, color: [u8; 4]);

    /// Clears the output surface
    fn clear(&mut self);

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn line(&mut self, a: (f64, f64), b: (f64, f64));

    /// Draws a rectangle bounded by two corners
    fn rectangle(&mut self, a: (f64, f64), b: (f64, f64));

    /// Returns the next pending event
    fn get_events(&mut self) -> Vec<Event>;

    /// Asks that the Drawable stop any tasks and cleanup
    fn close(&mut self) {} // provide empty default impl
}
