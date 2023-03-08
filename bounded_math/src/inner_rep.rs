use core::{
  fmt::{self, Debug, Display},
  hint::unreachable_unchecked,
  marker::Destruct,
};

use crate::RangeType;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IntRepr<const RANGE: RangeType>(<Self as SpecIntReprU0>::Repr);

#[const_trait]
pub trait IntRepresentation {
  type Repr: Clone + Copy + ~const TryFrom<u128> + ~const Into<u128>;
  fn to_i128(self) -> i128;
  unsafe fn from_i128_unchecked(val: i128) -> Self;
}

impl<const RANGE: RangeType> const IntRepresentation for IntRepr<RANGE> {
  type Repr = <Self as SpecIntReprU0>::Repr;
  #[inline]
  fn to_i128(self) -> i128 {
    let res = RANGE.start().checked_add_unsigned(self.0.into());
    debug_assert!(res.is_some());
    unsafe { res.unwrap_unchecked() }
  }
  #[inline]
  unsafe fn from_i128_unchecked(val: i128) -> Self
  where
    <<Self as SpecIntReprU0>::Repr as TryFrom<u128>>::Error: ~const Destruct,
  {
    #[allow(clippy::cast_sign_loss)]
    let offset = val.wrapping_sub(*RANGE.start()) as u128;
    let Some(offset) = offset.try_into().ok() else {
      unsafe {unreachable_unchecked()}
    };
    Self(offset)
  }
}
impl<const RANGE: RangeType> Debug for IntRepr<RANGE> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.to_i128(), f)
  }
}
impl<const RANGE: RangeType> Display for IntRepr<RANGE> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.to_i128(), f)
  }
}

trait CanHoldValueInner<const VAL: u128> {
  const RET: bool;
}

trait CanHoldValue<const VAL: u128> {}
impl<T, const VAL: u128> CanHoldValue<VAL> for T where T: CanHoldValueInner<VAL, RET = true> {}

const fn range_size(r: RangeType) -> u128 {
  r.end().abs_diff(*r.start())
}

macro_rules! repr_spec {
  ($rerp_trait:ident, (($largest_prim:ty, $largest_prim_tr:ident), $(($prim:ty, $prim_ty:ident)),*)) => {
    #[const_trait]
    #[doc(hidden)]
    pub trait $largest_prim_tr {
      type Repr: Clone + Copy + ~const TryFrom<u128> + ~const Into<u128>;
    }
    impl<const VAL: u128> CanHoldValueInner<VAL> for $largest_prim {
      const RET: bool = VAL <= Self::MAX as u128;
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
        type Repr: Clone + Copy + ~const TryFrom<u128> + ~const Into<u128>;
    }
    impl<const VAL: u128> CanHoldValueInner<VAL> for $cur_prim {
      const RET: bool = VAL <= Self::MAX.into();
    }

    impl<const RANGE: RangeType> const $cur_prim_tr for IntRepr<RANGE>
    where
      $cur_prim: CanHoldValue<{ range_size(RANGE) }>,
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
    (u128, SpecIntReprU128),
    (u64, SpecIntReprU64),
    (u32, SpecIntReprU32),
    (u16, SpecIntReprU16),
    (u8, SpecIntReprU8),
    (U0, SpecIntReprU0)
  )
);

#[non_exhaustive]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct U0;

impl U0 {
  pub const MAX: Self = Self;
  pub const MIN: Self = Self;
}
impl const From<U0> for u128 {
  #[inline]
  fn from(_: U0) -> Self {
    0
  }
}

impl const TryFrom<u128> for U0 {
  type Error = ();
  #[inline]
  fn try_from(value: u128) -> Result<Self, Self::Error> {
    if value == 0 {
      Ok(Self)
    } else {
      Err(())
    }
  }
}
