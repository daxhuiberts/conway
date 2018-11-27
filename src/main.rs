use std::fmt;

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
                next[x + y * self.width] = cell;
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Conway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+{}+\n", "--".repeat(self.width))?;

        for row in self.cells.chunks(self.width) {
            write!(f, "|")?;
            for cell in row {
                write!(f, "{}", if *cell { "()" } else { "  " })?;
            }
            write!(f, "|\n")?;
        }

        write!(f, "+{}+\n", "--".repeat(self.width))?;

        Ok(())
    }
}

fn main() {
    let mut conway = Conway::new(10, 10);

    conway.set(1, 1, true);
    conway.set(1, 3, true);
    conway.set(2, 2, true);
    conway.set(3, 1, true);
    conway.set(3, 3, true);

    println!("{}", conway);

    conway.tick();

    println!("{}", conway);
}
