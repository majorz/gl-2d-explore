#![allow(unused_must_use)]

extern crate gl;
extern crate glutin;
extern crate libc;

use std::ffi::CStr;
use std::str;

fn gl_string<'a>(name: u32) -> &'a str {
   unsafe {
      str::from_utf8(CStr::from_ptr(gl::GetString(name) as *const i8).to_bytes()).unwrap()
   }
}

fn print_gl_info(){
   let gl_version = gl_string(gl::VERSION);
   let glsl_version = gl_string(gl::SHADING_LANGUAGE_VERSION);
   let vendor = gl_string(gl::VENDOR);

   println!("OpenGL: {}\nGLSL: {}\nVENDOR: {}", gl_version, glsl_version, vendor);
}

fn main() {
   let window = glutin::Window::new().unwrap();

   unsafe { window.make_current() };

   unsafe {
      gl::load_with(|symbol| window.get_proc_address(symbol));

      gl::ClearColor(0.0, 1.0, 0.0, 1.0);
   }

   print_gl_info();

   for event in window.wait_events() {
      unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
      window.swap_buffers();

      match event {
         glutin::Event::Closed => break,
         _ => ()
      }
   }
}
