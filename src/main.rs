#![allow(unused_must_use)]

extern crate glutin;
extern crate libc;

use std::ffi::CStr;
use std::str;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

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
   let window = glutin::WindowBuilder::new().with_dimensions(250, 250).with_gl_profile(glutin::GlProfile::Compatibility).build().unwrap();

   unsafe { window.make_current() };

   println!("Pixel format of the window: {:?}", window.get_pixel_format());

   gl::load(&window);

   unsafe {
         gl::ClearColor(1., 1., 1., 1.);
         gl::Color3f(0., 1., 0.);
         gl::PointSize(10.);
         gl::MatrixMode(gl::PROJECTION);
         gl::LoadIdentity();
         gl::Ortho(0., 250., 0., 250., -1., 1.);
   }

   print_gl_info();

   for event in window.wait_events() {
      unsafe {
         gl::Clear(gl::COLOR_BUFFER_BIT);

         gl::Begin(gl::POINTS);
         gl::Vertex2i(40, 210);
         gl::End();
         gl::Flush();

         gl::Flush();
      };

      window.swap_buffers();

      match event {
         glutin::Event::Closed => break,
         _ => ()
      }
   }
}
