pub mod core;
#[path= "spot-vs-perp/mod.rs"]
pub mod spot_vs_perp;
pub use spot_vs_perp::algo::MakerAlgo;
#[path= "long-gamma-ddh/mod.rs"]
pub mod long_gamma_ddh;
pub use long_gamma_ddh::algo::GammaDDHAlgo;