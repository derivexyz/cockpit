//! A line search that stops after `max_iters`, as MINPACK's More-Thuente stops after `maxfev`
//! evaluations. argmin's More-Thuente keeps no such cap and BFGS runs it without `max_iters`, so a
//! search whose conditions are never met, e.g. on a NaN cost, spins forever.
use argmin::core::{ArgminFloat, Error, IterState, LineSearch, Problem, Solver, TerminationStatus, KV};

/// Well above the longest normal search seen (58 iterations), so only a runaway is cut.
pub const LINE_SEARCH_MAX_ITERS: u64 = 100;

#[derive(Clone)]
pub struct CappedLineSearch<L> {
    inner: L,
    max_iters: u64,
}

impl<L> CappedLineSearch<L> {
    pub fn new(inner: L, max_iters: u64) -> Self {
        Self { inner, max_iters }
    }
}

impl<D, F, L: LineSearch<D, F>> LineSearch<D, F> for CappedLineSearch<L> {
    fn search_direction(&mut self, direction: D) {
        self.inner.search_direction(direction)
    }

    fn initial_step_length(&mut self, step_length: F) -> Result<(), Error> {
        self.inner.initial_step_length(step_length)
    }
}

impl<O, L, P, G, F> Solver<O, IterState<P, G, (), (), (), F>> for CappedLineSearch<L>
where
    L: Solver<O, IterState<P, G, (), (), (), F>>,
    P: Clone,
    F: ArgminFloat,
{
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn init(
        &mut self,
        problem: &mut Problem<O>,
        state: IterState<P, G, (), (), (), F>,
    ) -> Result<(IterState<P, G, (), (), (), F>, Option<KV>), Error> {
        self.inner.init(problem, state.max_iters(self.max_iters))
    }

    fn next_iter(
        &mut self,
        problem: &mut Problem<O>,
        state: IterState<P, G, (), (), (), F>,
    ) -> Result<(IterState<P, G, (), (), (), F>, Option<KV>), Error> {
        self.inner.next_iter(problem, state)
    }

    fn terminate(&mut self, state: &IterState<P, G, (), (), (), F>) -> TerminationStatus {
        self.inner.terminate(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argmin::core::{CostFunction, Executor, Gradient, State, TerminationReason};
    use argmin::solver::linesearch::MoreThuenteLineSearch;
    use ndarray::{array, Array1};

    /// Finite only at the start: no step can meet the line search's conditions.
    struct NanAwayFromStart;

    impl CostFunction for NanAwayFromStart {
        type Param = Array1<f64>;
        type Output = f64;

        fn cost(&self, p: &Array1<f64>) -> Result<f64, Error> {
            Ok(if p[0] == 0.0 { 0.0 } else { f64::NAN })
        }
    }

    impl Gradient for NanAwayFromStart {
        type Param = Array1<f64>;
        type Gradient = Array1<f64>;

        fn gradient(&self, p: &Array1<f64>) -> Result<Array1<f64>, Error> {
            Ok(if p[0] == 0.0 { array![-1.0] } else { array![f64::NAN] })
        }
    }

    #[test]
    fn a_line_search_on_a_nan_cost_stops_at_its_cap() {
        // uncapped, argmin's More-Thuente spins here forever, as it did inside BFGS in the slice fitter
        let more_thuente = MoreThuenteLineSearch::<Array1<f64>, Array1<f64>, f64>::new().with_bounds(1e-6, 0.1).unwrap();
        let mut search = CappedLineSearch::new(more_thuente, 5);
        search.search_direction(array![1.0]);
        let result = Executor::new(NanAwayFromStart, search)
            .configure(|state| state.param(array![0.0]).gradient(array![-1.0]).cost(0.0))
            .run()
            .unwrap();
        assert_eq!(result.state.get_iter(), 5);
        assert_eq!(result.state.get_termination_reason(), Some(&TerminationReason::MaxItersReached));
    }
}
