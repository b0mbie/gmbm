#[cfg(not(feature = "rse-math"))]
mod no_rse;
#[cfg(not(feature = "rse-math"))]
pub use no_rse::*;

#[cfg(feature = "rse-math")]
pub use rse_math::{Vector, QAngle};
