#![allow(unused_attributes)]
#![allow(dead_code)]
#![feature(globs)]

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
    pub window_size: WindowSize,
    pub window_position: WindowPosition,
    pub border: WindowBorders,
}
impl Copy for WindowedMode{}

pub enum WindowMode{
    FullScreen(i32, i32),
    Windowed(WindowedMode),
}
impl Copy for WindowMode{}

pub struct OpenGLVersion{
    pub majour: i32,
    pub minor: i32,
}
impl Copy for OpenGLVersion{}

pub struct WindowOptions<'t>{
    pub title: &'t str,
    pub mode: WindowMode,
    pub gl_version: OpenGLVersion,
    pub red_bits: u8,
    pub green_bits: u8,
    pub blue_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,
}

