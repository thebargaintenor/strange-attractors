pub mod clifford;
pub mod color;
pub mod dejong;

pub trait Attract {
    fn next(&self, x: f64, y: f64) -> (f64, f64);
    fn rmax(&self) -> f64;
}
