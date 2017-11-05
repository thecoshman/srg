#![allow(dead_code)]
extern crate xlib;
extern crate libc;
#[link(name="GLX")]
extern crate glx;

use super::display;
use common;

pub struct X11Window{
    x_window: u64,
    display: display::Display,
}

pub fn create_simple() -> Option<X11Window>{
    let display = match display::open() {
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
            X11Window{ x_window: raw_id, display: display }
        }
    };

    return Some(window);
}

pub fn create(options:common::WindowOptions) -> Option<X11Window>{
    let display = match display::open() {
        Some(d) => d,
        None => return None,
    };
    find_best_frame_buffer_config(&display, &options);
    None
}

fn find_best_frame_buffer_config(display:&display::Display, options:&common::WindowOptions){
    let true_int = 1 as i32;
    let visual_attribs = [
        glx::X_RENDERABLE as i32,  true_int,
//       GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT,
//       GLX_RENDER_TYPE,   GLX_RGBA_BIT,
//       GLX_X_VISUAL_TYPE, GLX_TRUE_COLOR,
//       GLX_RED_SIZE,      options.redBits,  // 8
//       GLX_GREEN_SIZE,    options.greenBits, // 8
//       GLX_BLUE_SIZE,     options.blueBits, // 8
//       GLX_ALPHA_SIZE,    options.alphaBits, // 8
//       GLX_DEPTH_SIZE,    options.depthBits,  // 24
//       GLX_STENCIL_SIZE,  options.stencilBits, // 8
//       GLX_DOUBLEBUFFER,  true_int,
         glx::NONE as i32
    ];
    // let event = 0 as *mut xlib::XEvent;
    let frame_buffer_config_count = 0 as *mut libc::c_int;
    // pub fn ChooseFBConfig(   dpy: *mut types::Display, 
    //                          screen: __gl_imports::libc::c_int, 
    //                          attrib_list: *const __gl_imports::libc::c_int, 
    //                          nelements: *mut __gl_imports::libc::c_int) -> *mut types::GLXFBConfig;
    //auto frameBufferConfigs = 
    unsafe { glx::ChooseFBConfig(get_glx_display(display), display.default_screen(), visual_attribs[0] as *mut i32, frame_buffer_config_count); } // requires XFree
    
//    if (!frameBufferConfigs){
//        throw std::runtime_error("Failed to get frame buffer configurations");
//    }
//    int best = -1, bestNumberSamples = -1;//,worst_fbc = -1, worst_num_samp = 999;
//    for(int i = 0; i < frame_buffer_config_count; ++i){
//        XVisualInfo *visualInfo = glXGetVisualFromFBConfig(display, frameBufferConfigs[i]);
//        if (visualInfo){
//            int sampleBuffer, samples;
//            glXGetFBConfigAttrib(display, frameBufferConfigs[i], GLX_SAMPLE_BUFFERS, &sampleBuffer);
//            glXGetFBConfigAttrib(display, frameBufferConfigs[i], GLX_SAMPLES, &samples);
//            if (best < 0 || (sampleBuffer && (samples > bestNumberSamples))){
//                best = i, bestNumberSamples = samples;
//            }
//        }
//        XFree(visualInfo);
//    }
//    GLXFBConfig bestFrameBufferConfig = frameBufferConfigs[best];
//    //GLXFBConfig bestFrameBufferConfig = frameBufferConfigs[0]; // TODO update this so that it actually looks through the array to find 'best' config
//    XFree (frameBufferConfigs );
//    return bestFrameBufferConfig;
}

fn get_glx_display(display:&display::Display) -> *mut glx::types::Display{
    display.get_internal() as *mut glx::types::Display
}

impl X11Window {
    pub fn get_internal(&self) -> u64{
        self.x_window
    }

    pub fn show(&self){
        self.display.map_window(self);
        self.display.flush();
    }
}

