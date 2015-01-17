#![allow(dead_code)]
extern crate xlib;
extern crate libc;

use super::display::Display;

pub struct X11Window{
    x_window: u64,
}

pub fn create_simple() -> Option<X11Window>{
    let display = match Display::open() {
        Some(d) => d,
        None => return None,
    };

    let window = {
        let raw_id = {
            let root_window = unsafe { xlib::XRootWindow(display.get_internal(), 0) };
            unsafe { xlib::XCreateSimpleWindow(display.get_internal(), root_window, 50i32, 50i32, 640u32, 480u32, 1u32, 1u64, 1u64) }
        };
        if raw_id == 0 {
            return None
        } else {
            X11Window{ x_window: raw_id}
        }
    };

    display.map_window(&window);
    display.flush();

    return Some(window);
}

//pub fn create(d:Display) -> Option<X11Window>{
    //None
//}

impl X11Window {
    pub fn get_internal(&self) -> u64{
        self.x_window
    }
}
