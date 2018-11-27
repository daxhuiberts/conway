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
    let conway = Conway::new(10, 10);
    println!("{}", conway);
}
