mod conway;

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, WindowOptions, Window};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "width", default_value = "100")]
    width: usize,
    #[structopt(long = "height", default_value = "100")]
    height: usize,
    #[structopt(long = "fgcolor", default_value = "0")]
    fgcolor: u32,
    #[structopt(long = "bgcolor", default_value = "16777215")]
    bgcolor: u32,
    #[structopt(long = "speed", default_value = "20")]
    speed: u64,
    #[structopt(long = "fade", default_value = "0")]
    fade: u32,
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

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
                for xx in 0..opt.width {
                    conway.set(xx, y as usize, true);
                }
                for yy in 0..opt.height {
                    conway.set(x as usize, yy, true);
                }
            }
        }

        if !pauze {
            conway.tick();
            write_to_buffer(&conway, &mut buffer, opt.fgcolor, opt.bgcolor, opt.fade);
        }

        window.update_with_buffer(&buffer, opt.width, opt.height).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(opt.speed));
    }
}

fn write_to_buffer(conway: &conway::Conway, buffer: &mut [u32], fgcolor: u32, bgcolor: u32, fade: u32) {
    conway.cells().iter().zip(buffer).for_each(|(cell, pixel)|
        *pixel = if *cell {
            fgcolor
        } else {
            [0, 8, 16, 24].iter().map(|shift| {
                (((((*pixel >> shift) & 0xffu32) * fade) + ((bgcolor >> shift) & 0xffu32)) / (fade + 1)) << shift
            }).sum()
        }
    )
}
