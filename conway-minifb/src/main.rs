use conway_lib::Conway;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, ScaleMode, Window, WindowOptions};
use structopt::StructOpt;

const SIZE: usize = 8;

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

    let window_options = WindowOptions {
        resize: true,
        scale_mode: ScaleMode::AspectRatioStretch,
        ..WindowOptions::default()
    };
    let mut window = Window::new("Conway", opt.width * SIZE, opt.height * SIZE, window_options).unwrap();

    let mut conway = get_conway(None, get_window_size(&window));
    let mut buffer: Vec<u32> = vec![0; conway.width() * conway.height()];
    let mut pauze = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        conway = get_conway(Some(conway), get_window_size(&window));
        buffer.resize(conway.width() * conway.height(), 0);

        if window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
            pauze = !pauze;
        }

        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
                for xx in 0..conway.width() {
                    conway.set(xx, y as usize / SIZE, true);
                }
                for yy in 0..conway.height() {
                    conway.set(x as usize / SIZE, yy, true);
                }
            }
        }

        if !pauze {
            conway.tick();
            write_to_buffer(&conway, &mut buffer, opt.fgcolor, opt.bgcolor, opt.fade);
        }

        window
            .update_with_buffer(&buffer, conway.width(), conway.height())
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(opt.speed));
    }
}

fn get_window_size(window: &Window) -> (usize, usize) {
    let (width, height) = window.get_size();
    (width / SIZE, height / 8)
}

fn get_conway(conway: Option<Conway>, (width, height): (usize, usize)) -> Conway {
    conway.map_or_else(|| Conway::random(width, height), |conway|
        if width != conway.width() || height != conway.height() {
            Conway::random(width, height)
        } else {
            conway
        }
    )
}

fn write_to_buffer(
    conway: &conway_lib::Conway,
    buffer: &mut [u32],
    fgcolor: u32,
    bgcolor: u32,
    fade: u32,
) {
    conway.cells().iter().zip(buffer).for_each(|(cell, pixel)| {
        *pixel = if *cell {
            fgcolor
        } else {
            [0, 8, 16, 24]
                .iter()
                .map(|shift| {
                    (((((*pixel >> shift) & 0xffu32) * fade) + ((bgcolor >> shift) & 0xffu32))
                        / (fade + 1))
                        << shift
                })
                .sum()
        }
    })
}
