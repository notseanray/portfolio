use ode_solvers::{Vector3, Dop853};
use wasm_bindgen::prelude::wasm_bindgen;

type State = Vector3<f64>;
const Y0: State = State::new(-1.0, 1.0, 1.0);

#[wasm_bindgen]
pub fn lorenz(start_t: f64, step_size: f64, rho: f64, sigma: f64, beta: f64) -> Vec<f64> {
    // Define problem specific constants
    let system = LorenzAttractor {
        rho,
        sigma,
        beta,
    };
    // Create stepper and integrate
    let mut stepper = Dop853::new(system, start_t, start_t + step_size, 1e-3, Y0, 1e-4, 1e-4);
    let _ = stepper.integrate().unwrap();
    let mut results = vec![];
    for (x, v) in stepper.x_out().iter().zip(stepper.y_out().iter()) {
        for v in v {
            results.push(*v);
        }
    }
    results
}

struct LorenzAttractor {
    sigma: f64,
    beta: f64,
    rho: f64,
}

impl ode_solvers::System<State> for LorenzAttractor {
    fn system(&self, _t: f64, y: &State, dy: &mut State) {
        dy[0] = self.sigma * (y[1] - y[0]);
        dy[1] = y[0] * (self.rho - y[2]) - y[1];
        dy[2] = y[0] * y[1] - self.beta * y[2];
    }
}

