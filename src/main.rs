#[derive(Debug)]
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

fn main() {
    let conway = Conway::new(10, 10);
    println!("{:?}", conway);
}
