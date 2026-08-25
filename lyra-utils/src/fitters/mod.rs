pub mod slice_fitter;
pub mod surface_fitter;
pub mod svi;

/// Locally defined so this module stays free of crate-wide imports ahead of its migration out.
pub const SEC_PER_YEAR: f64 = 365.0 * 24.0 * 60.0 * 60.0;
