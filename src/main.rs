extern crate glutin;
extern crate gl;


fn main() {
    let window = glutin::Window::new().unwrap();

    unsafe { window.make_current() };

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol));

        gl::ClearColor(0.4, 0.6, 0.9, 1.0);
    }

    while !window.is_closed() {
        window.wait_events();

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        window.swap_buffers();
    }
}

