extern crate xlib;
extern crate libc;

use self::libc::types::common::c95;
use super::window;

pub struct Display{
    x_display: *mut xlib::Display,
}

impl Display{
    pub fn open() -> Option<Display>{
        let null = 0 as *mut i8;
        let not_opened_result = 0 as *mut c95::c_void;
        let display = unsafe { 
            xlib::XOpenDisplay(null)
        };
        if display == not_opened_result{
            None
        } else {
            Some(Display{ x_display: display})
        }
    }

    pub fn map_window(&self, window:window::X11Window){
        unsafe { xlib::XMapWindow(self.x_display, window.get_internal()) };
    }

    pub fn flush(&self){
        unsafe { xlib::XFlush(self.x_display) };
    }

    pub fn pending(&self) -> i32{
        unsafe { xlib::XPending(self.x_display) }
    }

    pub fn next_event(&self) -> *mut xlib::XEvent{
        let event = 0 as *mut xlib::XEvent;
        println!("Blocking untill event...");
        unsafe { xlib::XNextEvent(self.x_display, event)};
        println!("got an event!");
        return event;
    }

    pub fn get_internal(&self) -> *mut xlib::Display{
        self.x_display
    }
}
