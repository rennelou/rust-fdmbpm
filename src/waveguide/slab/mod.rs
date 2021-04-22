use crate::array::Array2d;
use super::core_waveguide::Core;
use super::*;
use super::eletric_field_2d;
use super::eletric_field_2d::EletricField2d;
use fp::list;
use fp::list::List;

pub struct Slab2d {
	pub xdelta: f64,
	pub zdelta: f64,
	pub xsteps: usize,
	pub zsteps: usize,
	kright: Complex<f64>,
	kleft:  Complex<f64>,
	s: List<List<Complex<f64>>>,
	q: List<List<Complex<f64>>>,
}

const X: usize = 0;
const Z: usize = 1;

pub fn new(g: &Array2d, r: &impl Core, n0: f64, k: f64, alpha: f64, kleft: Complex<f64>, kright: Complex<f64>) -> Slab2d {
    
	let xdelta = g.get(X).delta;
	let xsteps = g.get(X).steps;
	
	let zdelta = g.get(Z).delta;
	let zsteps = g.get(Z).steps;

    let guiding_space = |x: f64, z: f64| k.powf(2.0)*xdelta.powf(2.0)*(r.get_half_n(x, z, n0).powf(2.0)-n0.powf(2.0));
    let free_space = || 4.0*k*n0*xdelta.powf(2.0)/zdelta;
    let loss = |_, _| 2.0*k*n0*xdelta.powf(2.0)*alpha;
    
    let s = g.get(Z).get_points().map(
        |z| g.get(X).get_points().map(
            // okamoto 7.98
            |x| Complex::new(2.0 - guiding_space(x, z), free_space() + loss(x, z))
        ).collect()
    ).collect();
    
    let q = g.get(Z).get_points().map(
        |z| g.get(X).get_points().map(
            // okamoto 7.99
            |x| Complex::new(-2.0 + guiding_space(x, z), free_space() - loss(x, z))
        ).collect()
    ).collect();
    
    Slab2d{ xdelta,zdelta,xsteps,zsteps,kright,kleft,s,q }
}

impl Slab2d {

	pub fn fdmbpm(&self, e_input: List<Complex<f64>>) -> EletricField2d {
		
		let es = (1usize..self.zsteps).fold(
			vec![e_input], 
			|result, i| {
				
				let last_es = fp::last_or_default(&result, list::empty());
				let last_q = self.q[i-1].clone();
				
				let ds = get_ds(last_es, last_q);
				let abcs = self.get_abcs(i);

				let new_es = self.insert_boundary_values(
					i, 
					get_recurrence_form(get_alphas_betas(abcs, ds)
					)
				);

				return list::append(result, new_es);
			}
		);

		return eletric_field_2d::new(self, es);
	}

	fn get_abcs(&self, z: usize) -> List<Abc> {
		
		if self.xsteps >= MINIMALSTEP {
			
			let head = list::new( Abc {
				// okamoto 7.108a
				a: zero(), b: self.s[z][1] - self.left_boundary(z), c: one()
			});
			
			let body = (2..self.xsteps-2).map(
				// okamoto 7.108b
				|i| Abc { a: one(), b: self.s[z][i], c: one() }

			).collect();
			
			let last = list::new( Abc {
				/// okamoto 7.108c
				a: one(), 
				b: self.s[z][self.xsteps - 2usize] - self.right_boundary(z), 
				c: zero()
			});

			return list::concat(list::concat(head,body),last);
		}

		return list::empty();
	}

	fn insert_boundary_values(&self, z: usize, es: List<Complex<f64>>) -> List<Complex<f64>>{
		
		let head = list::new({
			let es_head = fp::head_or_default(&es, one());

			es_head*self.left_boundary(z) // okamoto 7.106

		});

		let last = list::new({
			let es_last = fp::last_or_default(&es, one());

			es_last*self.right_boundary(z) // okamoto 7.105

		});
		
		return list::concat(list::concat(head, es),last);
	}

	fn right_boundary(&self, _z: usize) -> Complex<f64> {
		
		//Dirichlet condition
		return zero()
		
		//return ( 
		//	Complex::new(0.0, -1.0)*self.kright*Complex::new(self.xdelta, 0.0) 
		//).exp();
	}

	fn left_boundary(&self, _z: usize) -> Complex<f64> {
		
		//Dirichlet condition
		return zero()
		
		//return ( 
		//	Complex::new(0.0, -1.0)*self.kleft*Complex::new(self.xdelta, 0.0) 
		//).exp();
	}
}

#[cfg(test)]
mod tests {
	
use core::f64::consts::PI;
use crate::*;
use super::*;
   	
	#[test]
   	fn assert_abcs_sizes() {
   	    for i in 1..10 {
			let grid = array::Array2d::new(100.0, i as f64, 2.0, 1.0);
			let r = core_waveguide::rectilinear::new(&grid, 3.4757, i as f64/2.0, i as f64/5.0);
   	        let w = slab::new(&grid, &r, 2.0*PI/1.55, 1.0, 0.2, zero(), zero());
			let got = w.get_abcs(0);
			assert_eq!(got.len(), w.xsteps-2usize);
   	    }
   	}
	
	#[test]
   	fn assert_ds_size() {

		for i in 5..500 {
			let es = (0..i).map(|_| one()).collect();
			let qs = (0..i).map(|_| one()).collect();

			let got = get_ds(es, qs);
			assert_eq!(got.len(), i-2usize);
		}
   	}
}

pub mod mock {
	use super::*;
   	use num::complex::Complex;

	pub fn get_zeros(i: i32) -> List<Complex<f64>> {
		return (0..i).map(|_| zero()).collect();
	}

	pub fn get_ones(i: i32) -> List<Complex<f64>> {
		return (0..i).map(|_| Complex::new(1.0, 0.0) ).collect();
	}
}