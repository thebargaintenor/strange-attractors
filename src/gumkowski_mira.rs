use super::Attract;

pub struct Attractor {
    a: f64,
    b: f64,
}

impl Attractor {
    pub fn new(a: f64, b: f64) -> Attractor {
        return Attractor { a, b };
    }

    fn f(&self, x: f64) -> f64 {
        return self.a * x + (2.0 * (1.0 - self.a) * x.powf(2.0) / (1.0 + x.powf(2.0)))
    }
}

impl Attract for Attractor {
    fn next(&self, x: f64, y: f64) -> (f64, f64) {
        let xprime = self.b * y + self.f(x);
        let yprime = self.f(xprime) - x;
        return (xprime, yprime)
    }

    fn rmax(&self) -> f64 {
        return 1.0 // I have no idea what the behavior here looks like
    }
}