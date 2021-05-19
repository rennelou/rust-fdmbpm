use super::*;
use Phasor;
use cores::Core;
use crate::fp::matrix::{self, Idx};
use crate::fp::{comprehension, list};

pub fn run(core: &impl Core<2>, k: f64, alpha: f64, e_input: Matrix<Phasor>, boundary_codition: fn()-> Phasor) -> EletricField {
	let shape = core.get_shape().clone();
	let grid_steps = core.get_deltas().to_vec();
	let zsteps = shape[0];

	let (s, q) = get_initialized_params_2d(core, k, alpha);

	let es = (1usize..zsteps).fold(
		vec![e_input], 
		|result, i| {
			
			let last_es= fp::last(result.iter()).unwrap().view(&[Idx::Free]);
			let last_q = q.view(&[Idx::Value(i-1), Idx::Free]);

			let s_list = s.view(&[Idx::Value(i-1), Idx::Free]);

			let ds = get_ds(last_es, last_q);
			let d_list = ds.view(&[Idx::Free]);

			let new_es = insert_boundary_values(
				get_recurrence_form(get_alphas_betas(s_list, d_list, boundary_codition)),
				boundary_codition
			);

			return list::append(result, new_es);
		}
	);

	let values = matrix::zip(es);
	return EletricField { values, grid_steps };
}

pub fn get_initialized_params_2d(core: &impl Core<2>, k: f64, alpha: f64) -> (Matrix<Phasor>, Matrix<Phasor>) {
	let [zdepht, xdepht] = core.get_shape().clone();
	let [zdelta, xdelta] = core.get_deltas().clone();
	
	let n0 = core.get_n0();

    let guiding_space = |x: f64, z: f64| k.powf(2.0)*xdelta.powf(2.0)*(core.get_half_n(z,0.0, x, n0).powf(2.0)-n0.powf(2.0));
    let free_space = || 4.0*k*n0*xdelta.powf(2.0)/zdelta;
    let loss = |_, _| 2.0*k*n0*xdelta.powf(2.0)*alpha;
    
    let s = comprehension::arange(zdepht, zdelta).map(
        |z| comprehension::arange(xdepht, xdelta).map(
            
			// okamoto 7.98
            |x| Complex::new(2.0 - guiding_space(x, z), free_space() + loss(x, z))
        
		).collect()
    ).collect();
    
    let q = comprehension::arange(zdepht, zdelta).map(
        |z| comprehension::arange(xdepht, xdelta).map(
            
			// okamoto 7.99
            |x| Complex::new(-2.0 + guiding_space(x, z), free_space() - loss(x, z))
        
		).collect()
    ).collect();
    
	let shape = core.get_shape().to_vec();
    (fp::new_2d(s,&shape), fp::new_2d(q, &shape))
}