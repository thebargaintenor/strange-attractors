use array2d::Array2D;

use strange_attractors::{clifford, color};

fn main() {
    // TODO: move parameters into cli args (and makefile)
    let a = 2.0;
    let b = 2.0;
    let c = 1.0;
    let d = -1.0;

    let w: i32 = 1000;
    let h: i32 = 1000;

    let delta = 0.5;

    let max_steps = 100000;

    let max_color: color::Rgb8 = (100, 100, 255);

    let mut viewport = Array2D::filled_with(0.0, h as usize, w as usize);
    let attractor = clifford::Attractor::new(a, b, c, d);
    let max_amplitude = attractor.rmax();
    //eprintln!("RMax = {}", max_amplitude);

    let mut x: f64 = 100.0;
    let mut y: f64 = 100.0;
    for _ in 1..max_steps {
        let (xn, yn) = attractor.next(x, y);
        let (x_view, y_view) = transform_to_viewport(&viewport, max_amplitude, xn, yn);

        if x_view >= 0 && x_view < w && y_view >= 0  && y_view < h {
            viewport[(x_view as usize, y_view as usize)] += delta;
        }

        x = xn;
        y = yn;
    }

    write_ppm(&viewport, max_color);
}

fn transform_to_viewport(viewport: &Array2D<f64>, max_amplitude: f64, x: f64, y: f64) -> (i32, i32) {
    let w = viewport.num_columns() as f64;
    let h = viewport.num_rows() as f64;

    let constraint_size = w.min(h);
    let scale = (0.9 * constraint_size) / (max_amplitude * 2.0);

    let tx = scale * x + w / 2.0;
    let ty = scale * y + h / 2.0;

    return (tx.round() as i32, ty.round() as i32)
}

fn write_ppm(viewport: &Array2D<f64>, max_color: color::Rgb8) {
    println!("P3\n{} {}\n255", viewport.num_columns(), viewport.num_rows());

    for px in viewport.elements_row_major_iter() {
        let (r, g, b) = color::at_gradient_step((0, 0, 0), max_color, *px);
        println!("{} {} {}", r, g, b);
    }
}
