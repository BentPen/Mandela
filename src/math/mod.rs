
pub use num::complex::Complex64;

/// z_{n+1} = (z_n)^2 + c
pub struct MandelbrotMap {
    c: Vec<Complex64>,
    z: Vec<Complex64>,
    n: usize
}

impl MandelbrotMap {
    pub fn new(c: Vec<Complex64>) -> Self {
        let z = c.clone();
        let n = 0;
        Self { c, z, n }
    }
    pub fn get_c(&self) -> &Vec<Complex64> {
        &self.c
    }
    pub fn get_z(&self) -> &Vec<Complex64> {
        &self.z
    }

    pub fn step(&mut self) {
        let c = self.c.clone();
        for (idx, z) in self.z.iter_mut().enumerate() {
            *z = (*z)*(*z) + c[idx];
        }
        self.n += 1;
    }
    
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_step() {
        let new_vec = vec![Complex64{re:1.0, im:2.0}, Complex64{re:2.0, im:3.0}];
        let mut mandelbrot = MandelbrotMap::new(new_vec);
        let z1 = mandelbrot.get_z();
        for z in z1 {
            println!("Before: {}, {}", z.re, z.im);
        }
        mandelbrot.step();
        let z2 = mandelbrot.get_z();
        for z in z2 {
            println!("After: {}, {}", z.re, z.im);
        }
    }
}
