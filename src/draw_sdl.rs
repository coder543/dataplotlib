#![allow(dead_code)]
#![allow(unused_variables)]

use sdl2::{self, Sdl, EventPump};
use sdl2::render::Renderer;
use sdl2::event::Event as SdlEvent;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;

use draw::*;

pub struct DrawSDL {
    context: Sdl,
    events: EventPump,
    renderer: Renderer<'static>,
    screenspace: (Range, Range),
    color: pixels::Color,
}

impl DrawSDL {
    pub fn new() -> DrawSDL {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window("2D plot", 720, 720)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let events = sdl_context.event_pump().unwrap();
        let renderer = window.renderer().build().unwrap();

        let default = Range {
            min: 0.0,
            max: 0.0,
        };

        DrawSDL {
            context: sdl_context,
            events: events,
            renderer: renderer,
            screenspace: (default, default),
            color: pixels::Color::RGBA(0, 0, 0, 255),
        }
    }
}

impl Drawable for DrawSDL {
    /// Sets the visible range of worldspace
    fn set_view(&mut self, x: Range, y: Range) {}

    fn set_color(&mut self, color: [u8; 4]) {
        self.color = pixels::Color::RGBA(color[0], color[1], color[2], color[3]);
    }

    fn clear(&mut self) {
        self.renderer.set_draw_color(self.color);
        self.renderer.clear();
    }

    /// Draws a line from (x, y) -> (x, y) in worldspace
    fn line(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64)) {

        // need to use point2plot!

        self.renderer
            .thick_line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, 2, self.color)
            .unwrap();
    }

    /// Draws a rectangle bounded by two corners
    fn rectangle(&mut self, (x1, y1): (f64, f64), (x2, y2): (f64, f64)) {

        // need to use point2plot!

        self.renderer
            .rectangle(x1 as i16, y1 as i16, x2 as i16, y2 as i16, self.color)
            .unwrap();
    }

    /// Returns the next pending event
    fn get_events(&mut self) -> Vec<Event> {

        let mut events = Vec::new();

        for event in self.events.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => return vec![Event::Quit],

                SdlEvent::KeyDown { keycode: Some(keycode), .. } => {
                    events.push(Event::Keydown(keycode as i32));
                }
                SdlEvent::Window { win_event: sdl2::event::WindowEvent::Resized(new_w, new_h), .. } => {
                    events.push(Event::Resize(new_w as f64, new_h as f64));
                }
                _ => {}
            }
        }

        return events;
    }

    /// Asks that the Drawable stop any tasks and cleanup
    fn close(&mut self) {
        // placeholder, not needed for Sdl2 backend
    }
}
