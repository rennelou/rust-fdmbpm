use super::*;

pub struct Rectilinear<const N: usize> {
    pub shape: [usize;N],
    pub deltas: [f64;N],
    pub position: f64,
    n: f64,
    n0: f64,
    core_left: f64,
    core_right: f64
}

pub fn new_2d(dx: f64, xdelta: f64, dz: f64, zdelta: f64, n: f64, n0: f64, position: f64, core_width: f64) -> Rectilinear<2usize> {
    if position >= dx|| core_width >= dx {
        panic!("percent parameters need be less than 1");
    }

    let zsteps = (dz/zdelta).round() as usize;
    let xsteps = (dx/xdelta).round() as usize;
    let shape = [zsteps, xsteps];

    let deltas = [zdelta, xdelta];

    let core_left = position - (core_width/2.0);
    let core_right = position + (core_width/2.0);
    
    Rectilinear { shape, deltas, position, n, n0, core_left, core_right }
}

impl<const N: usize> Core<N> for Rectilinear<N> {
    
    fn get_shape(&self) -> [usize;N] {
        self.shape
    }

    fn get_deltas(&self) -> [f64;N] {
        self.deltas
    }

    fn get_n(&self, x: f64, _: f64, n0: f64) -> f64 {
        return {
            if x > self.core_left && x < self.core_right {
                self.n
            } else {
                n0
            }
        }
    }

    // Essa função ta incompleta
    fn get_half_n(&self, x: f64, z: f64, n0: f64) -> f64 {
        return self.get_n(x, z, n0);
    }

    fn get_n0(&self) -> f64 {
        return self.n0;
    }
}

