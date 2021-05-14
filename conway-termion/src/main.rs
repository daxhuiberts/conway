use std::io::{Write, stdout};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use termion::screen::AlternateScreen;
use termion::terminal_size;

const ALIVE_STR: &'static str = "()";
const DEAD_STR: &'static str = "  ";

fn main() {
    let (width, height) = terminal_size().unwrap();
    let width = width as usize / 2;
    let height = height as usize;
    let delay = 50;

    let mut game = conway_lib::Conway::new(width, height);
    game.randomize();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.clone().store(false, Ordering::SeqCst);
    }).unwrap();

    let mut screen = AlternateScreen::from(stdout());

    while running.load(Ordering::SeqCst) {
        sleep(std::time::Duration::from_millis(delay));
        game.tick();
        write_game(&mut screen, &game);
    }

    fn write_game(screen: &mut AlternateScreen<std::io::Stdout>, game: &conway_lib::Conway) {
        game.each_cell_alive(false, |x, y, alive| {
            write!(screen, "{}", if alive { ALIVE_STR } else { DEAD_STR }).unwrap();
            if x + 1 == game.width() && y + 1 != game.height() {
                write!(screen, "\n").unwrap();
            }
        });

        screen.flush().unwrap();
    }
}
