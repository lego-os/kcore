#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
mod arm;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub use arm::*;
// #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
// pub use riscv::*;
pub mod mm;