use core::{
  cmp::Ordering,
  ops::{Add, Div, Mul, Rem, Sub},
};

use crate::{Integer, IntegerRange, RangeType};

macro_rules! impl_op {
    ($op_tr:ident, $op_fn_name:ident | $lhs_name:ident, $rhs_name:ident | {$($body:tt)+}) => {
      #[doc(hidden)]
      mod $op_fn_name {
        use super::*;
        pub const fn extend_bounds(
          $lhs_name: RangeType,
          $rhs_name: RangeType,
        ) -> RangeType {
          debug_assert!(!$lhs_name.is_empty() && !$rhs_name.is_empty()); // Guaranteed by the type system
          $($body)+
        }
      }

      impl<RHS: ~const IntegerRange, const LHS_RANGE: RangeType>
        const $op_tr<RHS> for Integer<LHS_RANGE>
      where
        Integer<{ $op_fn_name::extend_bounds(LHS_RANGE, RHS::RANGE) }>:
      {
        type Output = Integer<{ $op_fn_name::extend_bounds(LHS_RANGE, RHS::RANGE) }>;

        #[inline]
        fn $op_fn_name(self, rhs: RHS) -> Self::Output {
          let res = Self::Output::try_from($op_tr::$op_fn_name(self.get_value(), rhs.get_value()));
          debug_assert!(res.is_ok());
          let res = unsafe {res.ok().unwrap_unchecked()};
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
  i128::min(i128::min(lhs.start() * rhs.start(), lhs.start() * rhs.end()), i128::min(lhs.end() * rhs.start(), lhs.end() * rhs.end()))
  ..=
  i128::max(i128::max(lhs.start() * rhs.start(), lhs.start() * rhs.end()), i128::max(lhs.end() * rhs.start(), lhs.end() * rhs.end()))
}}

impl_op! {Div, div |lhs, rhs| {
  assert!(!(*rhs.start() == 0 && *rhs.end() == 0), "tried to divide by Integer<0..=0>");
  let effective_end = if *rhs.end() == 0 { -1 } else {*rhs.end()};
  let effective_start = if *rhs.start() == 0 { 1 } else {*rhs.start() };

  let val1 = *lhs.start() / effective_end;
  let val2 = *lhs.end() / effective_end;

  let mut min = val1.min(val2);
  let mut max = val1.max(val2);

  let val1 = *lhs.start() / effective_start;
  let val2 = *lhs.end() / effective_start;

  min = min.min(val1.min(val2));
  max = max.max(val1.max(val2));

  if effective_start != 1 && rhs.contains(&1) {
    let val1 = *lhs.start();
    let val2 = *lhs.end();

    min = min.min(val1.min(val2));
    max = max.max(val1.max(val2));
  };
  if effective_end != -1 && rhs.contains(&-1) {
    let val1 = -*lhs.start();
    let val2 = -*lhs.end();

    min = min.min(val1.min(val2));
    max = max.max(val1.max(val2));
  };

  min..=max

}}

impl_op! {Rem, rem |lhs, rhs| {
  assert!(!(*rhs.start() == 0 && *rhs.end() == 0), "tried to get remainder of division by Integer(0..=0)");
  let eff_rhs = rhs.start().abs().max(rhs.end().abs());

  let start = if *lhs.start() < 0 { -(eff_rhs - 1) } else {0};
  let end = if *lhs.end() > 0 {eff_rhs - 1} else {0};

  start..=end
}}

const fn ranges_overlap<RHS: ~const IntegerRange, const LHS_RANGE: RangeType>() -> bool {
  let lhs_contains_rhs_start = LHS_RANGE.contains(RHS::RANGE.start());
  let lhs_contains_rhs_end = LHS_RANGE.contains(RHS::RANGE.end());
  let rhs_contains_lhs_start = RHS::RANGE.contains(LHS_RANGE.start());
  lhs_contains_rhs_start || lhs_contains_rhs_end || rhs_contains_lhs_start
}

impl<RHS: ~const IntegerRange, const LHS_RANGE: RangeType> const PartialEq<RHS>
  for Integer<LHS_RANGE>
{
  #[inline]
  fn eq(&self, other: &RHS) -> bool {
    const fn must_be_equal<RHS: ~const IntegerRange, const LHS_RANGE: RangeType>() -> bool {
      let lhs_exact = LHS_RANGE.start() == LHS_RANGE.end();
      let rhs_exact = RHS::RANGE.start() == RHS::RANGE.end();
      let same_start = LHS_RANGE.start() == RHS::RANGE.start();
      lhs_exact && rhs_exact && same_start
    }

    if const { must_be_equal::<RHS, LHS_RANGE>() } {
      true
    } else if const { !ranges_overlap::<RHS, LHS_RANGE>() } {
      false
    } else {
      self.get_value().eq(&other.get_value())
    }
  }
}

impl<RHS: ~const IntegerRange, const LHS_RANGE: RangeType> const PartialOrd<RHS>
  for Integer<LHS_RANGE>
{
  #[inline]
  fn partial_cmp(&self, other: &RHS) -> Option<Ordering> {
    if const { ranges_overlap::<RHS, LHS_RANGE>() } {
      self.get_value().partial_cmp(&other.get_value())
    } else {
      // Since we already know that the ranges don't overlap it doesn't matter which bound we compare from which Range
      if RHS::RANGE.start() < LHS_RANGE.start() {
        Some(Ordering::Less)
      } else {
        Some(Ordering::Greater)
      }
    }
  }
}
