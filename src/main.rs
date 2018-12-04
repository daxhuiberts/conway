extern crate minifb;
extern crate rand;

mod conway;

use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

fn main() {
    let mut conway = conway::Conway::new(WIDTH, HEIGHT);
    conway.randomize();

    let window_options = WindowOptions { scale: Scale::X8, ..WindowOptions::default() };
    let mut window = Window::new("Conway", WIDTH, HEIGHT, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    write_to_buffer(&conway, &mut buffer);
    window.update_with_buffer(&buffer).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(std::time::Duration::from_millis(20));

        conway.tick();
        write_to_buffer(&conway, &mut buffer);
        window.update_with_buffer(&buffer).unwrap();
    }
}

fn write_to_buffer(conway: &conway::Conway, buffer: &mut [u32]) {
    conway.cells().iter().zip(buffer).for_each(|(cell, pixel)|
        *pixel = if *cell { 0x00000000 } else { 0x00ffffff }
    )
}
