use core::ops::{Add, Div, Mul, RangeInclusive, Rem, Sub};

use crate::{InnerType, Integer, IntegerRange, RangeIsEmpty};

macro_rules! impl_op {
    ($op_tr:ident, $op_fn_name:ident | $lhs_name:ident, $rhs_name:ident | {$($body:tt)+}) => {
      #[doc(hidden)]
      mod $op_fn_name {
        use super::*;
        pub const fn extend_bounds(
          $lhs_name: RangeInclusive<InnerType>,
          $rhs_name: RangeInclusive<InnerType>,
        ) -> RangeInclusive<InnerType> {
          debug_assert!(!$lhs_name.is_empty() && !$rhs_name.is_empty()); // Guaranteed by the type system
          $($body)+
        }
      }

      impl<RHS: ~const IntegerRange, const LHS_RANGE: RangeInclusive<InnerType>>
        const $op_tr<RHS> for Integer<LHS_RANGE>
      where
        (): RangeIsEmpty<LHS_RANGE, RET = false>,
        (): RangeIsEmpty<{ RHS::RANGE }, RET = false>,
        (): RangeIsEmpty<{ $op_fn_name::extend_bounds(LHS_RANGE, RHS::RANGE) }, RET = false>,
        InnerType: ~const From<RHS>
      {
        type Output = Integer<{ $op_fn_name::extend_bounds(LHS_RANGE, RHS::RANGE) }>;

        fn $op_fn_name(self, rhs: RHS) -> Self::Output {
          let res = Integer {
            val: $op_tr::$op_fn_name(self.val, InnerType::from(rhs)),
          };
          debug_assert!({ $op_fn_name::extend_bounds(LHS_RANGE, RHS::RANGE) }.contains(&res.get_value()));
          // Not usable in const: debug_assert!({ $op_fn_name::extend_bounds(LHS_RANGE, RHS::RANGE) }.contains(&res.get_value()), "Invalid result: {}({:?}, {:?}) = {:?}", stringify!($op_fn_name), self, rhs.to_integer(), res);
          res
        }
      }
    };
  }
impl_op! {Add, add |lhs, rhs| { (lhs.start() + rhs.start())..=(lhs.end() + rhs.end()) }}
impl_op! {Sub, sub |lhs, rhs| {
  (lhs.start() - rhs.end())
  ..=
  (lhs.end() - rhs.start())
}}
impl_op! {Mul, mul |lhs, rhs| {
  InnerType::min(InnerType::min(lhs.start() * rhs.start(), lhs.start() * rhs.end()), InnerType::min(lhs.end() * rhs.start(), lhs.end() * rhs.end()))
  ..=
  InnerType::max(InnerType::max(lhs.start() * rhs.start(), lhs.start() * rhs.end()), InnerType::max(lhs.end() * rhs.start(), lhs.end() * rhs.end()))
}}

impl_op! {Div, div |lhs, rhs| {
  if *rhs.start() == 0 && *rhs.end() == 0 {
    panic!("tried to divide by Integer<0..=0>");
  }
  let effective_end = if *rhs.end() != 0 { *rhs.end() } else {-1};
  let effective_start = if *rhs.start() != 0 { *rhs.start() } else {1};

  let val1 = *lhs.start() / effective_end;
  let val2 = *lhs.end() / effective_end;

  let mut min = val1.min(val2);
  let mut max = val1.max(val2);

  let val1 = *lhs.start() / effective_start;
  let val2 = *lhs.end() / effective_start;

  min = min.min(val1.min(val2));
  max = max.max(val1.max(val2));

  if effective_start != 1 && rhs.contains(&1) {
    let val1 = *lhs.start() / 1;
    let val2 = *lhs.end() / 1;

    min = min.min(val1.min(val2));
    max = max.max(val1.max(val2));
  };
  if effective_end != -1 && rhs.contains(&-1) {
    let val1 = *lhs.start() / -1;
    let val2 = *lhs.end() / -1;

    min = min.min(val1.min(val2));
    max = max.max(val1.max(val2));
  };

  min..=max

}}

impl_op! {Rem, rem |lhs, rhs| {
  if *rhs.start() == 0 && *rhs.end() == 0 {
    panic!("tried to get remainder of division by Integer(0..=0)");
  }
  let eff_rhs = rhs.start().abs().max(rhs.end().abs());

  let start = if *lhs.start() < 0 { -(eff_rhs - 1) } else {0};
  let end = if *lhs.end() > 0 {eff_rhs - 1} else {0};

  start..=end
}}

// TODO these could be optimized since the return value is always the same if the two ranges don't overlap.
impl<RHS: ~const IntegerRange, const LHS_RANGE: RangeInclusive<InnerType>> const PartialEq<RHS>
  for Integer<LHS_RANGE>
where
  (): RangeIsEmpty<LHS_RANGE, RET = false>,
  (): RangeIsEmpty<{ RHS::RANGE }, RET = false>,
{
  fn eq(&self, other: &RHS) -> bool {
    self.get_value().eq(&other.get_value())
  }
}

impl<RHS: ~const IntegerRange, const LHS_RANGE: RangeInclusive<InnerType>> const PartialOrd<RHS>
  for Integer<LHS_RANGE>
where
  (): RangeIsEmpty<LHS_RANGE, RET = false>,
  (): RangeIsEmpty<{ RHS::RANGE }, RET = false>,
{
  fn partial_cmp(&self, other: &RHS) -> Option<std::cmp::Ordering> {
    self.get_value().partial_cmp(&other.get_value())
  }
}
