#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unexpected_cfgs)]

// cannot use 2D and 3D features at the same time
#[cfg(all(feature = "2d", feature = "3d"))]
compile_error!("The '2d' & '3d' features cannot be used at the same time.");

/// Define the data layout used for each tile in the map
pub mod tile;
