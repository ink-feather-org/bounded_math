#![no_std]
#![feature(const_option_ext)]
#![feature(const_slice_index)]
#![feature(const_mut_refs)]
#![feature(error_in_core)]
#![feature(const_trait_impl)]
#![feature(const_maybe_uninit_write)]
#![feature(const_cmp)]
#![feature(inline_const)]
#![feature(const_convert)]

mod biguint;

pub use biguint::BigUInt;

#[cfg(test)]
mod test;
