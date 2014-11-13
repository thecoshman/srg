#![allow(dead_code)]
extern crate xlib;
extern crate libc;

//use super::display;

pub struct X11window{
    x_window: u64,
}

pub fn create_simple() -> Option<X11Window>{
    let display = display::Open();
    let display = match display {
        Some(d) => d,
        None => return None,
    };

    let root_window = unsafe { xlib::XRootWindow(display.get_internal(), 0) };
    let window = unsafe { xlib::XCreateSimpleWindow(display.get_internal(), root_window, 50i32, 50i32, 640u32, 480u32, 1u32, 1u64, 1u64) };
    if window == 0 {
        None
    } else {
        Some(X11Window{ x_window: window})
    }
}

pub fn create(display:display::Display) -> Option<X11Window>{
    None
}

impl X11Window {
    pub fn get_internal(&self) -> u64{
        self.x_window
    }
}
