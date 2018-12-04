extern crate minifb;
extern crate rand;
#[macro_use]
extern crate structopt;

mod conway;

use minifb::{Key, KeyRepeat, Scale, WindowOptions, Window};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "width", default_value = "100")]
    width: usize,
    #[structopt(long = "height", default_value = "100")]
    height: usize,
    #[structopt(long = "speed", default_value = "20")]
    speed: u64,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut conway = conway::Conway::new(opt.width, opt.height);
    conway.randomize();

    let window_options = WindowOptions { scale: Scale::X8, ..WindowOptions::default() };
    let mut window = Window::new("Conway", opt.width, opt.height, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; opt.width * opt.height];

    let mut pauze = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
            pauze = !pauze;
        }

        if !pauze {
            conway.tick();
            write_to_buffer(&conway, &mut buffer);
        }

        window.update_with_buffer(&buffer).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(opt.speed));
    }
}

fn write_to_buffer(conway: &conway::Conway, buffer: &mut [u32]) {
    conway.cells().iter().zip(buffer).for_each(|(cell, pixel)|
        *pixel = if *cell { 0x00000000 } else { 0x00ffffff }
    )
}
