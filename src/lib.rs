// Import dependencies
// use lazy_static::lazy_static;
// use libc;
// use rand;
// use test::stats::Stats;
// use std;
// extern crate rand;

// Modules are other .rs source files
mod bootstrap;

// Export functions called by R
pub use bootstrap::bootstrap;
pub use bootstrap::mean;
pub use bootstrap::stdev;
pub use bootstrap::sterr;
