pub type Rgb8 = (u8, u8, u8);

pub fn at_gradient_step(start: Rgb8, end: Rgb8, step: f64) -> Rgb8 {
    if step <= 0.0 {
        return start
    }
    if step >= 1.0 {
        return end
    }

    let (r1, g1, b1) = start;
    let (r2, g2, b2) = end;

    let r = f64::from(r1) + f64::from(r2 - r1) * step;
    let g = f64::from(g1) + f64::from(g2 - g1) * step;
    let b = f64::from(b1) + f64::from(b2 - b1) * step;

    return (
        r.min(255.0).round() as u8,
        g.min(255.0).round() as u8,
        b.min(255.0).round() as u8,
    )
}