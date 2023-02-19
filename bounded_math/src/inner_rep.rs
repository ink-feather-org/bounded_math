use std::{
  fmt::{Debug, Display},
  marker::Destruct,
};

use crate::{RangeInRange, RangeType};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IntRepr<const RANGE: RangeType>(<Self as SpecIntRepri8>::Repr);

#[const_trait]
pub trait IntRepresentation {
  type Repr: Clone + Copy + Debug + Display + ~const TryFrom<i128> + ~const Into<i128>;
  fn to_i128(self) -> i128;
  fn from_i128(val: i128) -> Self;
}

impl<const RANGE: RangeType> const IntRepresentation for IntRepr<RANGE> {
  type Repr = <Self as SpecIntRepri8>::Repr;
  fn to_i128(self) -> i128 {
    self.0.into()
  }
  fn from_i128(val: i128) -> Self
  where
    <<IntRepr<RANGE> as SpecIntRepri8>::Repr as TryFrom<i128>>::Error: ~const Destruct,
  {
    if RANGE.contains(&val) {
      let Some(val) = val.try_into().ok() else {
           unreachable!()
        };
      Self(val)
    } else {
      panic!("Tried to convert to IntReprt from an out of range value")
    }
  }
}
impl<const RANGE: RangeType> Debug for IntRepr<RANGE> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <<Self as SpecIntRepri8>::Repr as Debug>::fmt(&self.0, f)
  }
}
impl<const RANGE: RangeType> Display for IntRepr<RANGE> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <<Self as SpecIntRepri8>::Repr as Display>::fmt(&self.0, f)
  }
}

macro_rules! repr_spec {
  ($rerp_trait:ident, (($largest_prim:ty, $largest_prim_tr:ident), $(($prim:ty, $prim_ty:ident)),*)) => {
    #[const_trait]
    #[doc(hidden)]
    pub trait $largest_prim_tr {
      type Repr: Clone + Copy + Debug + Display + ~const TryFrom<i128> + ~const Into<i128>;
    }
    impl<const RANGE: RangeType> const $largest_prim_tr for IntRepr<RANGE>
    {
      type Repr = $largest_prim;
    }
    repr_spec!{
      $rerp_trait,
      $largest_prim_tr,
      ($(($prim, $prim_ty)),*)
    }
  };
  ($rerp_trait:ident, $last_spec_tr:ident, (($cur_prim:ty, $cur_prim_tr:ident)$(, $(($prim:ty, $prim_tr:ident)),+)?)) => {
    #[const_trait]
    #[doc(hidden)]
    pub trait $cur_prim_tr {
        type Repr: Clone + Copy + Debug + Display + ~const TryFrom<i128> + ~const Into<i128>;
    }

    impl<const RANGE: RangeType> const $cur_prim_tr for IntRepr<RANGE>
    where
      $cur_prim: RangeInRange<RANGE, CONTAINED = true>,
    {
      type Repr = $cur_prim;
    }

    impl<const RANGE: RangeType> const $cur_prim_tr for IntRepr<RANGE> {
      default type Repr = <Self as $last_spec_tr>::Repr;
    }
    repr_spec!{
        $rerp_trait,
        $cur_prim_tr,
        ($($(($prim, $prim_tr)),*)?)
      }
  };

  ($rerp_trait:ident, $last_spec_tr:ident, ()) => {}
}
repr_spec!(
  IntRepresentation,
  (
    (i128, SpecIntRepri128),
    (i64, SpecIntRepri64),
    (i32, SpecIntRepri32),
    (i16, SpecIntRepri16),
    (i8, SpecIntRepri8)
  )
);
