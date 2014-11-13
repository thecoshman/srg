#![allow(unused_attributes)]
#![allow(dead_code)]
#![feature(globs)]

// pub use x11::window;


pub enum WindowSize{
    Maximised,
    Resolution(i32, i32),
}
impl Copy for WindowSize{}

pub enum WindowPosition{
    Centered,
    Position(i32, i32),
}
impl Copy for WindowPosition{}

pub enum WindowBorders{
    On,
    Off,
}
impl Copy for WindowBorders{}

pub struct WindowedMode{
    window_size: WindowSize,
    window_position: WindowPosition,
    border: WindowBorders,
}
impl Copy for WindowedMode{}

pub enum WindowMode{
    FullScreen(i32, i32),
    Windowed(WindowedMode),
}
impl Copy for WindowMode{}

pub struct OpenGLVersion{
    majour: i32,
    minor: i32,
}
impl Copy for OpenGLVersion{}

pub struct WindowOptions{
    title: String,
    mode: WindowMode,
    gl_version: OpenGLVersion,
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
}

