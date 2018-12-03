extern crate minifb;

use minifb::{Key, KeyRepeat, Scale, WindowOptions, Window};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

struct Conway {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Conway {
    pub fn new(width: usize, height: usize) -> Self {
        Conway {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.cells[x + y * self.height]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.cells[x + y * self.height] = value;
    }

    pub fn tick(&mut self) {
        let mut next = vec![false; self.width * self.height];

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get(x, y);
                let live_neighbors = self.live_neighbor_count(x, y);
                let new_cell = Self::compute_state(cell, live_neighbors);
                next[x + y * self.width] = new_cell;
            }
        }

        self.cells = next;
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        offsets.iter().filter(|(xoffset, yoffset)| {
            let x = (x as isize + xoffset + self.width as isize) % self.width as isize;
            let y = (y as isize + yoffset + self.height as isize) % self.height as isize;
            self.get(x as usize, y as usize)
        }).count()
    }

    fn compute_state(current_state: bool, live_neighbors: usize) -> bool {
        live_neighbors == 3 || (live_neighbors == 2 && current_state == true)
    }

    fn write_to_buffer(&self, buffer: &mut [u32]) {
        self.cells.iter().zip(buffer).for_each(|(cell, pixel)|
            *pixel = if *cell { 0x00000000 } else { 0x00ffffff }
        )
    }
}

fn main() {
    let mut conway = Conway::new(WIDTH, HEIGHT);

    conway.set(0, 1, true);
    conway.set(1, 2, true);
    conway.set(2, 0, true);
    conway.set(2, 1, true);
    conway.set(2, 2, true);

    let window_options = WindowOptions { scale: Scale::X16, ..WindowOptions::default() };
    let mut window = Window::new("Conway", WIDTH, HEIGHT, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    conway.write_to_buffer(&mut buffer);
    window.update_with_buffer(&buffer).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
            conway.tick();
            conway.write_to_buffer(&mut buffer);
            window.update_with_buffer(&buffer).unwrap();
        } else {
            window.update();
        }
    }
}
