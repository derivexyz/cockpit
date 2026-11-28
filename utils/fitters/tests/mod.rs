mod clock;
mod fit_multithread;
mod forward_var;
mod slice_fitter;
mod surface_fitter;

/// When the tests' quotes are taken.
const REF_SEC: i64 = 1_000_000;

fn expiry(days: i64) -> i64 {
    REF_SEC + days * 86_400
}
