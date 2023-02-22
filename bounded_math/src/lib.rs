#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]
#![feature(generic_arg_infer)]
#![feature(adt_const_params)]
#![feature(const_cmp)]
#![feature(const_range_bounds)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(specialization)]
#![feature(const_result_drop)]
#![feature(const_option)]
#![feature(const_try)]
#![feature(const_option_ext)]
#![feature(const_refs_to_cell)]
#![cfg_attr(test, feature(test, const_num_from_num))]
#![allow(incomplete_features)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::use_self)]

mod impls;
mod num;

mod inner_rep;

type RangeType = RangeInclusive<i128>;

use std::ops::RangeInclusive;

pub use num::{aliases::*, Integer, IntegerRange, RangeInRange};

#[cfg(test)]
mod test;
