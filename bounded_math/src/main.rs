#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]
#![feature(generic_arg_infer)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use crate::num::*;
mod cmp;
mod num;

type InnerType = i128;

fn main() {
  let a = 5;

  let exact_5 = Integer::<5, _>::new_exact();
  let ranged_5 = Integer::<100, 120>::new::<101>();

  let x = exact_5 * ranged_5;
  dbg!(x);
  let b = Integer::<-1, 300>::from(1u8);
  let c = <u8 as TryInto<NiceU8>>::try_into(a).unwrap() + b;
  dbg!(&c);
  let d = c.grow_bounds::<-3, 700>();
  dbg!(d);

  let e = c.try_change_bounds::<5, 5>();
  dbg!(e);

  let x = NiceI64::new::<5>();
  dbg!(&x);

  let y = x + NiceI64::new::<5>();
  dbg!(&y);
}
