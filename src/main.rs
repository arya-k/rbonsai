// use clap::Parser;
use console::{style, Term};
use rand::Rng;

const DEPTH: usize = 6;
const FILL_CHAR: [char; 16] = [
    ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
];

#[derive(Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn delta(&self, angle: f32, distance: usize) -> Self {
        Self {
            x: (self.x as f32 + (angle.cos() * distance as f32)) as usize,
            y: (self.y as f32 + (angle.sin() * distance as f32)) as usize,
        }
    }

    fn within(&self, height: usize, width: usize) -> bool {
        let (h, w) = (height as usize, width as usize);
        self.x > 0 && self.x < w && self.y > 0 && self.y < h
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Pixel {
    Empty,
    LargeBranch,
    SmallerBranch,
    Leaf,
}

struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Vec<Pixel>>,
    rng: rand::rngs::ThreadRng,
}

impl Canvas {
    fn from_term(term: &Term) -> Option<Self> {
        term.size_checked().map(|(height, width)| {
            let (h, w) = (height as usize * 2, width as usize * 2);
            Self {
                width: w,
                height: h,
                data: vec![vec![Pixel::Empty; w]; h],
                rng: rand::thread_rng(),
            }
        })
    }

    fn drawline(&mut self, p0: &Coordinate, p1: &Coordinate, pixel: &Pixel) {
        let (dy, dx) = (p1.y as i32 - p0.y as i32, p1.x as i32 - p0.x as i32);
        let n = i32::max(dy.abs(), dx.abs());

        let div_n: f32 = if n == 0 { 0.0 } else { 1.0 / n as f32 };
        let (x_step, y_step) = (dx as f32 * div_n, dy as f32 * div_n);

        let (mut x, mut y) = (p0.x as f32, p0.y as f32);
        for _ in 0..n {
            let c = Coordinate::new(x.round() as usize, y.round() as usize);
            if c.within(self.height, self.width) {
                self.data[c.y as usize][c.x as usize] = pixel.clone();
            }

            x += x_step;
            y += y_step;
        }
    }

    fn write(&self, term: Term) -> std::io::Result<()> {
        let filled = |y: usize, x: usize| match self.data[y][x] {
            Pixel::Empty => 0u8,
            _ => 1,
        };

        for y in (0..self.height).step_by(2) {
            let line = (0..self.width)
                .step_by(2)
                .map(|x| {
                    let block = filled(y, x)
                        | (filled(y, x + 1) << 1)
                        | (filled(y + 1, x) << 2)
                        | (filled(y + 1, x + 1) << 3);

                    let char = FILL_CHAR[block as usize];

                    let points: [_; 4] = [(y, x), (y, x + 1), (y + 1, x), (y + 1, x + 1)];
                    if points.iter().any(|(y, x)| self.data[*y][*x] == Pixel::Leaf) {
                        style(char).green()
                    } else {
                        style(char).yellow()
                    }
                })
                .map(|s| s.to_string())
                .collect::<String>();
            term.write_line(&line)?;
        }
        term.flush()
    }
}

#[derive(Debug)]
struct Branch {
    /// The coordinate of the branch
    pos: Coordinate,
    /// Current angle of the branch
    angle: f32,
    /// Length of the branch
    length: usize,
    /// Number of parents this branch has
    depth: usize,
}

impl Branch {
    /// Attempts to draw the branch until the length drops to zero. Every now and then,
    /// it will spawn a new branch off of it. This function returns all those new branches.
    pub fn draw(&mut self, canvas: &mut Canvas) -> Vec<Self> {
        let pixel = match self.depth {
            0 => Pixel::LargeBranch,
            1 | 2 | 3 => Pixel::SmallerBranch,
            _ => Pixel::Leaf,
        };

        let mut children = vec![];
        for _ in (0..self.length).step_by(2) {
            let end = self.pos.delta(self.angle, 2);
            canvas.drawline(&self.pos, &end, &pixel);

            // tweak the angle by a random amount
            self.pos = end;
            self.angle += canvas.rng.gen_range(-0.3..0.3);

            // Consider spawning a new branch
            if canvas.rng.gen_range(0.0..100.0) < 25.0 {
                children.push(Self {
                    pos: self.pos.clone(),
                    angle: self.angle + canvas.rng.gen_range(-1.0..1.0),
                    length: (self.length as f32 * 3. / 5.).round() as usize,
                    depth: self.depth + 1,
                });
            }
        }
        children
    }
}

fn draw_tree(canvas: &mut Canvas) {
    (0..DEPTH).fold(
        vec![Branch {
            pos: Coordinate::new((canvas.width / 2) as usize, (canvas.height - 1) as usize),
            angle: -0.5 * std::f32::consts::PI,
            length: (canvas.height * 2 / 3) as usize,
            depth: 0,
        }],
        |mut q, _| q.iter_mut().flat_map(|b| b.draw(canvas)).collect(),
    );
}

fn main() {
    // Initialize the terminal; fill it with chars
    let term = Term::buffered_stdout();
    // Draw the tree:
    let mut canvas = Canvas::from_term(&term).expect("Failed to get terminal dimensions");
    draw_tree(&mut canvas);
    canvas.write(term).expect("Failed to write to terminal");
}

// TODO:
// Make the leaves more full
// Thicker branches
// Vector field to affect the shape of the tree
