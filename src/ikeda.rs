use super::Attract;

use num_complex::Complex;

pub struct Attractor {
    a: f64,
    b: f64,
    k: f64,
    p: f64,
}

impl Attractor {
    pub fn new(a: f64, b: f64, k: f64, p: f64) -> Attractor {
        return Attractor { a, b, k, p };
    }
}

impl Attract for Attractor {
    fn next(&self, x: f64, y: f64) -> (f64, f64) {
        let z = Complex::new(x, y);
        let ik = Complex::new(0.0, self.k);
        let ip = Complex::new(0.0, self.p);

        // zn+1 = a + b zn exp[ i k - i p / (1 + | zn2 | ) ]
        let zprime = self.a + self.b * z * (ik + ip.unscale(1.0 + z.powi(2).norm())).exp();

        return (zprime.re, zprime.im);
    }

    fn rmax(&self) -> f64 {
        return 2.0;
    }
}