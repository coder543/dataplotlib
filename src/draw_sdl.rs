#![allow(dead_code)]
#![allow(unused_variables)]

use sdl2_mt::{self, Sdl, EventPump};
use sdl2_mt::event::Event as SdlEvent;
use sdl2_mt::pixels;

use sdl2_mt::Sdl2Mt;

use draw::*;

pub struct DrawSDL {
    sdlh: Sdl2Mt,
    window_id: u32,
    screenspace: (Range, Range),
    color: pixels::Color,
}

impl DrawSDL {
    pub fn new(sdlh: Sdl2Mt) -> DrawSDL {
        let window_id = sdlh.create_simple_window("2D plot", 720, 720).unwrap();

        let default = Range { min: 0.0, max: 0.0 };

        DrawSDL {
            sdlh,
            window_id,
            screenspace: (default, default),
            color: pixels::Color::RGBA(0, 0, 0, 255),
        }
    }
}

impl Drawable for DrawSDL {
    /// Sets the visible range of worldspace
    fn set_view(&mut self, x: Range, y: Range) {
        self.screenspace = (x, y);
    }

    /// Gets the visible range of worldspace
    fn get_view(&self) -> (Range, Range) {
        self.screenspace
    }

    /// Set color for various drawing actions
    fn set_color(&mut self, color: [u8; 4]) {
        self.color = pixels::Color::RGBA(color[0], color[1], color[2], color[3]);
    }

    /// Clears the output surface
    fn clear(&mut self) {
        let window_id = self.window_id;
        let color = self.color;
        self.sdlh.run_on_ui_thread(Box::new(move |_sdl, windows| {
            let canvas = windows.get_mut(&window_id).unwrap();
            canvas.set_draw_color(color);
            canvas.clear();
            canvas.present();
        }));
    }

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn line(&mut self, p1: (f64, f64), p2: (f64, f64)) {
        self.thick_line(p1, p2, 1);
    }

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn thick_line(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64), thickness: u16) {

        // need to use point2plot!

        let window_id = self.window_id;
        let color = self.color;
        self.sdlh.run_on_ui_thread(Box::new(move |_sdl, windows| {
            let canvas = windows.get_mut(&window_id).unwrap();
            canvas.set_draw_color(color);
            canvas.draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32));
            canvas.present();
        }));
    }

    /// Draws a rectangle bounded by two corners
    fn rectangle(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64)) {

        // need to use point2plot!

        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let h = (y2 as i32 - y1) as u32;
        let w = (x2 as i32 - x1) as u32;

        use sdl2_mt::rect::Rect;

        let window_id = self.window_id;
        let color = self.color;
        self.sdlh.run_on_ui_thread(Box::new(move |_sdl, windows| {
            let canvas = windows.get_mut(&window_id).unwrap();
            canvas.set_draw_color(color);
            canvas.draw_rect(Rect::new(x1, y1, w, h));
            canvas.present();
        }));
    }

    /// Returns the next pending event
    fn get_events(&mut self) -> Vec<Event> {

        use std::sync::mpsc::channel;

        let (tx, rx) = channel();

        let window_id = self.window_id;

        self.sdlh.handle_ui_events(
            Box::new(move |_sdl, _windows, event| {
                match event {
                    &SdlEvent::Quit { .. } => tx.send(Event::Quit).unwrap(),

                    &SdlEvent::KeyDown {
                        window_id: window_id,
                        keycode: Some(keycode),
                        ..
                    } => {
                        tx.send(Event::KeyDown(keycode as i32));
                    }
                    &SdlEvent::Window {
                        window_id: window_id,
                        win_event: sdl2_mt::event::WindowEvent::Resized(new_w, new_h),
                        ..
                    } => {
                        tx.send(Event::Resize(new_w as f64, new_h as f64));
                    }
                    _ => return false,
                }
                true
            }),
        );

        return rx.iter().collect();
    }
}
