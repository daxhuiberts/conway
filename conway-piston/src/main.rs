use conway_lib::Conway;
use piston_window::*;
// use std::collections::VecDeque;
// use std::time::{Duration, Instant};

const SIZE: usize = 8;

fn main() {
    let width = 200;
    let height = 110;
    let delay = 10;
    let delay = delay as f64 / 1000.0;

    let mut conway = Conway::new(width, height);
    conway.randomize();

    // let mut fps: VecDeque<Instant> = VecDeque::with_capacity(128);
    let mut time = 0f64;
    // let mut total_time = 0f64;

    let window_settings = WindowSettings::new(
        "Hello Piston!",
        [(width * SIZE) as u32, (height * SIZE) as u32],
    );
    let mut window: PistonWindow = window_settings.exit_on_esc(true).build().unwrap();

    // let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // let ref font = assets.join("FiraSans-Regular.ttf");
    // let factory = window.factory.clone();
    // let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut event_settings = window.get_event_settings();
    event_settings.set_ups(200);
    event_settings.set_ups_reset(2);
    event_settings.set_max_fps(60);
    window.set_event_settings(event_settings);

    // let start = Instant::now();

    // let mut rendered = Duration::new(0, 0);
    // let mut updated = Duration::new(0, 0);

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            // let render_begin = Instant::now();

            window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0, 1.0, 1.0, 1.0], graphics);

                conway.each_cell_alive(true, |x, y, alive| {
                    let color = if alive {
                        [0.0, 0.0, 0.0, 1.0]
                    } else {
                        [1.0, 1.0, 0.0, 1.0]
                    };
                    let position = [
                        (x * SIZE) as f64,
                        (y * SIZE) as f64,
                        SIZE as f64,
                        SIZE as f64,
                    ];
                    rectangle(color, position, context.transform, graphics);
                });

                // let now = Instant::now();
                // let a_second_ago = now - Duration::from_secs(1);
                //
                // while fps.front().map_or(false, |t| *t < a_second_ago) {
                //     fps.pop_front();
                // }
                //
                // fps.push_back(now);
                // fps.len();

                // let elapsed = start.elapsed();

                // let string = format!("fps: {}; elasped: {}; iteration: {}", fps.len(), elapsed.as_secs(), conway.get_count());
                // text([1.0, 0.0, 0.0, 1.0], 20, &string, &mut glyphs, context.transform.trans(0.0, 20.0), graphics).unwrap();
            });

            // rendered += render_begin.elapsed();
        }
        if let Some(update) = event.update_args() {
            time += update.dt;
            // total_time += update.dt;
            while time > delay {
                // let update_begin = Instant::now();
                conway.tick();
                // updated += update_begin.elapsed();
                time -= delay;
            }
        }
    }

    // println!("rendered: {:?}; updated: {:?}; time: {:?}", rendered, updated, total_time);
}
