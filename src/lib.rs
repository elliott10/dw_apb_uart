//! Uart snps,dw-apb-uart driver in Rust for boards:
//! BST A1000b,
//! T-HEAD C910,
//! RK3588

#![no_std]
#![allow(non_snake_case)]

pub mod dw_apb_uart;
pub use dw_apb_uart::*;

#[cfg(feature = "board_rk3588")]
pub mod utils;
