pub struct Attractor {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Attractor {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Attractor {
        return Attractor {a, b, c, d};
    }

    pub fn next(&self, x: f64, y: f64) -> (f64, f64) {
        let xprime = (self.a * y).sin() + self.c * (self.a * x).cos();
        let yprime = (self.b * x).sin() + self.d * (self.b * y).cos();

        return (xprime, yprime);
    }

    pub fn rmax(&self) -> f64 {
        return (1.0 + self.c.abs()).max(1.0 + self.d.abs());
    }
}