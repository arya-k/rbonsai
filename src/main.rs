use raqote::{DrawOptions, DrawTarget, SolidSource, Source};
use viuer::terminal_size;

struct Branch {
    id: u32,
    leaf: bool,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,

    // parameters
    ratio: f32,
    spread: f32,
    splitsize: f32,
    depth: u32,

    // size + direction data
    dir: Vec3d,
    length: f32,
    radius: f32,
    area: f32,
}

fn main() {
    let mut dt = {
        let (width, height) = terminal_size();
        DrawTarget::new(width as i32 * 2, height as i32 * 4)
    };

    // practice rendering things:
    dt.fill_rect(
        0.0,
        0.0,
        100.0,
        10.0,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 0, 0)),
        &DrawOptions::new(),
    );

    // now render to the terminal:
    dt.write_png("test.png").unwrap();
}

/*
THE PLAN:

- create a general pipeline to take a high pixel density canvas, and alias/rasterize it onto the
  terminal.
- implement transport-oriented tree growth:
  source: https://nickmcd.me/2020/10/19/transport-oriented-growth-and-procedural-trees/
- tweak the args till it looks like a bonsai (perhaps expose options with click?)
- for cosmetics, adding an age parameter to each branch would also allow for leaves to bloom over
  time. For this, we can look to how cbonsai does it.
- coloring schemes
- maybe: add a gradient field parameter to sway the tree in wind (for that cascading style)
- profit??
*/
