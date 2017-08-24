use image::{self, ImageBuffer};
use draw::*;


pub struct DrawImage {
    context: ImageBuffer,
    screenspace: Range2d,
    realspace: Range2d,
    color: [u8; 4],
}
#![allow(dead_code)]
#![allow(unused_variables)]

impl DrawImage {
    pub fn new(sdlh: Sdl2Mt) -> Box<DrawSDL> {
        let window_id = sdlh.create_simple_window("2D plot", 720, 720).unwrap();

        let default_s = Range { min: 0.0, max: 0.0 };

        let default_r = Range {
            min: 0.0,
            max: 720.0,
        };

        Box::new(DrawSDL {
            sdlh,
            window_id,
            screenspace: Range2d(default_s, default_s),
            realspace: Range2d(default_r, default_r),
            color: pixels::Color::RGBA(0, 0, 0, 255),
        })
    }
}

impl Drawable for DrawImage {
    /// Sets the visible range of worldspace
    fn set_view(&mut self, view: Range2d) {
        self.screenspace = view;
    }

    /// Gets the visible range of worldspace
    fn get_view(&self) -> Range2d {
        self.screenspace
    }

    /// Set color for various drawing actions
    fn set_color(&mut self, color: [u8; 4]) {
        self.color = pixels::Color::RGBA(color[0], color[1], color[2], color[3]);
    }

    /// Clears the output surface
    fn clear(&mut self) {
        let color = self.color;
//        self.sdlh
//            .run_on_ui_thread(Box::new(move |_sdl, windows| {
//                let canvas = windows.get_mut(&window_id).unwrap();
//                canvas.set_draw_color(color);
//                canvas.clear();
//            }))
//            .unwrap();
    }

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn line(&mut self, p1: (f64, f64), p2: (f64, f64)) {
        self.thick_line(p1, p2, 1);
    }

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn thick_line(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64), thickness: u16) {

        let x1 = point2window(x1, self.screenspace.0, self.realspace.0, false);
        let y1 = point2window(y1, self.screenspace.1, self.realspace.1, true);

        let x2 = point2window(x2, self.screenspace.0, self.realspace.0, false);
        let y2 = point2window(y2, self.screenspace.1, self.realspace.1, true);

        let color = self.color;
//        self.sdlh
//            .run_on_ui_thread(Box::new(move |_sdl, windows| {
//                let canvas = windows.get_mut(&window_id).unwrap();
//                canvas.set_draw_color(color);
//                canvas
//                    .draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))
//                    .unwrap();
//            }))
//            .unwrap();
    }

    /// Draws a rectangle bounded by two corners
    fn rectangle(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64)) {

        let x1 = point2window(x1, self.screenspace.0, self.realspace.0, false);
        let y1 = point2window(y1, self.screenspace.1, self.realspace.1, false);

        let x2 = point2window(x2, self.screenspace.0, self.realspace.0, false);
        let y2 = point2window(y2, self.screenspace.1, self.realspace.1, false);

        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let w = (x2 as i32 - x1) as u32;
        let h = (y2 as i32 - y1) as u32;

        let color = self.color;
//        self.sdlh
//            .run_on_ui_thread(Box::new(move |_sdl, windows| {
//                let canvas = windows.get_mut(&window_id).unwrap();
//                canvas.set_draw_color(color);
//                canvas.fill_rect(Rect::new(x1, y1, w, h)).unwrap();
//            }))
//            .unwrap();
    }

    /// Draws a rectangle bounded by two corners
    fn unfilled_rectangle(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64)) {

        let x1 = point2window(x1, self.screenspace.0, self.realspace.0, false);
        let y1 = point2window(y1, self.screenspace.1, self.realspace.1, false);

        let x2 = point2window(x2, self.screenspace.0, self.realspace.0, false);
        let y2 = point2window(y2, self.screenspace.1, self.realspace.1, false);

        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let w = (x2 as i32 - x1) as u32;
        let h = (y2 as i32 - y1) as u32;

        let color = self.color;
//        self.sdlh
//            .run_on_ui_thread(Box::new(move |_sdl, windows| {
//                let canvas = windows.get_mut(&window_id).unwrap();
//                canvas.set_draw_color(color);
//                canvas.draw_rect(Rect::new(x1, y1, w, h)).unwrap();
//            }))
//            .unwrap();
    }

    fn present(&mut self) {
//        self.sdlh
//            .run_on_ui_thread(Box::new(move |_sdl, windows| {
//                let canvas = windows.get_mut(&window_id).unwrap();
//                canvas.present();
//            }))
//            .unwrap();
    }

    /// Returns the next pending event
    fn get_events(&mut self) -> Vec<Event> {
        vec![Event::Quit]
    }
}
