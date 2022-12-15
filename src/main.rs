use array2d::Array2D;
use clap::{Args, Parser, Subcommand};

use strange_attractors::{clifford, color, dejong, ikeda, Attract, gumkowski_mira};

#[derive(Parser)]
#[clap(name = "strange-attractors")]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    global: GlobalOpts,

    #[clap(subcommand)]
    attractor: Attractors,
}

#[derive(Args)]
struct GlobalOpts {
    #[clap(short, long, global = true, default_value_t = 1000)]
    width: i32,

    #[clap(short, long, global = true, default_value_t = 1000)]
    height: i32,

    #[clap(short, long, global = true, default_value_t = 10000)]
    steps: i32,
}

#[derive(Subcommand)]
enum Attractors {
    Clifford {
        #[clap(short)]
        a: f64,

        #[clap(short)]
        b: f64,

        #[clap(short)]
        c: f64,

        #[clap(short)]
        d: f64,
    },
    DeJong {
        #[clap(short)]
        a: f64,

        #[clap(short)]
        b: f64,

        #[clap(short)]
        c: f64,

        #[clap(short)]
        d: f64,
    },
    Ikeda {
        #[clap(short)]
        a: f64,

        #[clap(short)]
        b: f64,

        #[clap(short)]
        k: f64,

        #[clap(short)]
        p: f64,
    },
    /// requires a and b in 0 < x < 1
    GumkowskiMira {
        #[clap(short)]
        a: f64,

        #[clap(short)]
        b: f64,
    }
}

fn main() {
    let args = Cli::parse();

    let w: i32 = args.global.width;
    let h: i32 = args.global.height;

    let delta = 0.5;
    let max_steps = args.global.steps;
    let max_color: color::Rgb8 = (100, 100, 255);

    let mut viewport = Array2D::filled_with(0.0, h as usize, w as usize);
    let attractor = load_attractor(args.attractor);
    let max_amplitude = attractor.rmax();

    let mut x: f64 = 100.0;
    let mut y: f64 = 100.0;
    for _ in 1..max_steps {
        let (xn, yn) = attractor.next(x, y);
        let (x_view, y_view) = transform_to_viewport(&viewport, max_amplitude, xn, yn);

        if x_view >= 0 && x_view < w && y_view >= 0 && y_view < h {
            viewport[(x_view as usize, y_view as usize)] += delta;
        }

        x = xn;
        y = yn;
    }

    write_ppm(&viewport, max_color);
}

fn load_attractor(attractor_opts: Attractors) -> Box<dyn Attract> {
    match &attractor_opts {
        &Attractors::Clifford { a, b, c, d } => Box::new(clifford::Attractor::new(a, b, c, d)),
        &Attractors::DeJong { a, b, c, d } => Box::new(dejong::Attractor::new(a, b, c, d)),
        &Attractors::Ikeda {a, b, k, p} => Box::new(ikeda::Attractor::new(a, b, k, p)),
        &Attractors::GumkowskiMira { a, b } => Box::new(gumkowski_mira::Attractor::new(a, b)),
    }
}

fn transform_to_viewport(
    viewport: &Array2D<f64>,
    max_amplitude: f64,
    x: f64,
    y: f64,
) -> (i32, i32) {
    let w = viewport.num_columns() as f64;
    let h = viewport.num_rows() as f64;

    let constraint_size = w.min(h);
    let scale = (0.9 * constraint_size) / (max_amplitude * 2.0);

    let tx = scale * x + w / 2.0;
    let ty = scale * y + h / 2.0;

    return (tx.round() as i32, ty.round() as i32);
}

fn write_ppm(viewport: &Array2D<f64>, max_color: color::Rgb8) {
    println!(
        "P3\n{} {}\n255",
        viewport.num_columns(),
        viewport.num_rows()
    );

    for px in viewport.elements_row_major_iter() {
        let (r, g, b) = color::at_gradient_step((0, 0, 0), max_color, *px);
        println!("{} {} {}", r, g, b);
    }
}
