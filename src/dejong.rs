use super::Attract;

pub struct Attractor {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Attractor {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Attractor {
        return Attractor { a, b, c, d };
    }
}

impl Attract for Attractor {
    fn next(&self, x: f64, y: f64) -> (f64, f64) {
        let xprime = (self.a * y).sin() - (self.b * x).cos();
        let yprime = (self.c * x).sin() - (self.d * y).cos();

        return (xprime, yprime);
    }

    fn rmax(&self) -> f64 {
        return 2.0;
    }
}
