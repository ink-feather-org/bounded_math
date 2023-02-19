use core::fmt::Debug;
use std::marker::Destruct;

use crate::{
  inner_rep::{IntRepr, IntRepresentation},
  RangeType,
};
pub trait RangeIsEmpty<const RANGE: RangeType> {
  const RET: bool;
}
impl<const RANGE: RangeType> RangeIsEmpty<RANGE> for () {
  const RET: bool = RANGE.is_empty();
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Integer<const RANGE: RangeType>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  val: IntRepr<RANGE>,
}

trait ContainsRet<const VALUE: i128> {
  const RET: bool;
}

impl<const RANGE: RangeType, const VALUE: i128> ContainsRet<VALUE> for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  const RET: bool = RANGE.contains(&VALUE);
}

pub trait ValInRange<const VALUE: i128> {}
impl<const RANGE: RangeType, const VALUE: i128> ValInRange<VALUE> for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
  Self: ContainsRet<VALUE, RET = true>,
{
}
pub trait RangeInRange<const CONTAINED_RANGE: RangeType> {
  const CONTAINED: bool;
}
impl<T: IntegerRange, const CONTAINED_RANGE: RangeType> RangeInRange<CONTAINED_RANGE> for T {
  const CONTAINED: bool =
    T::RANGE.contains(CONTAINED_RANGE.start()) && T::RANGE.contains(CONTAINED_RANGE.end());
}

impl<const RANGE: RangeType> Debug for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    write!(
      f,
      "Integer<{}..={}>: {}",
      RANGE.start(),
      RANGE.end(),
      self.val
    )
  }
}
pub trait IsExact {
  const EXACT: bool;
}
impl<const RANGE: RangeType> IsExact for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  const EXACT: bool = RANGE.start() == RANGE.end();
}

impl<const RANGE: RangeType> Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  #[must_use]
  pub const fn new<const VALUE: i128>() -> Self
  where
    Self: ValInRange<VALUE>,
  {
    Self {
      val: IntRepr::<RANGE>::from_i128(VALUE),
    }
  }
  #[must_use]
  pub const fn new_exact() -> Self
  where
    Self: IsExact<EXACT = true>,
  {
    Self {
      val: IntRepr::<RANGE>::from_i128(*RANGE.start()),
    }
  }
}
trait RangeNotEmpty {}

#[const_trait]
pub trait IntegerRange: Copy + ~const Into<i128> {
  const RANGE: RangeType;

  fn get_value(self) -> i128;

  fn to_integer(self) -> Integer<{ Self::RANGE }>
  where
    (): RangeIsEmpty<{ Self::RANGE }, RET = false>,
  {
    let Ok(ret) = self.get_value().try_into() else {
      unreachable!()
    };
    ret
  }

  fn to<T: ~const IntegerRange + ~const TryFrom<i128>>(self) -> T
  where
    (): RangeIsEmpty<{ Self::RANGE }, RET = false> + RangeIsEmpty<{ T::RANGE }, RET = false>,
    Integer<{ T::RANGE }>: RangeInRange<{ Self::RANGE }, CONTAINED = true>,
    Result<T, T::Error>: ~const Destruct,
  {
    let Ok(ret) = self.get_value().try_into() else {
        unreachable!()
    };
    ret
  }
  fn try_to<T: ~const IntegerRange + ~const From<i128>>(self) -> Option<T>
  where
    (): RangeIsEmpty<{ Self::RANGE }, RET = false> + RangeIsEmpty<{ T::RANGE }, RET = false>,
  {
    if Self::RANGE.contains(T::RANGE.start()) && Self::RANGE.contains(T::RANGE.end())
      || Self::RANGE.contains(&self.into())
    {
      Some(self.get_value().into())
    } else {
      None
    }
  }
}

impl<const RANGE_GEN: RangeType> const IntegerRange for Integer<RANGE_GEN>
where
  (): RangeIsEmpty<RANGE_GEN, RET = false>,
{
  const RANGE: RangeType = RANGE_GEN;

  fn get_value(self) -> i128 {
    self.val.to_i128()
  }
}

impl<const RANGE: RangeType> const From<Integer<RANGE>> for i128
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  fn from(value: Integer<RANGE>) -> Self {
    value.val.to_i128()
  }
}
impl<const RANGE: RangeType> const From<i128> for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  fn from(value: i128) -> Self {
    Self {
      val: IntRepr::<RANGE>::from_i128(value),
    }
  }
}
pub mod aliases {
  //use core::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64};

  use super::{Integer, IntegerRange, RangeType};
  macro_rules! aliases {
    ($nice_name:ident, $base_type:ty) => {
      impl const IntegerRange for $base_type {
        const RANGE: RangeType = <$base_type>::MIN as i128..=<$base_type>::MAX as i128;
        fn get_value(self) -> i128 {
          self.into()
        }
      }

      pub type $nice_name = Integer<{ <$base_type>::RANGE }>;
    };
  }

  aliases!(NiceU8, u8);
  aliases!(NiceU16, u16);
  aliases!(NiceU32, u32);
  aliases!(NiceU64, u64);
  //aliases!(NiceU128, u128);

  aliases!(NiceI8, i8);
  aliases!(NiceI16, i16);
  aliases!(NiceI32, i32);
  aliases!(NiceI64, i64);
  aliases!(NiceI128, i128);

  //aliases!(NiceNonZeroU8, NonZeroU8);
  //aliases!(NiceNonZeroU16, NonZeroU16);
  //aliases!(NiceNonZeroU32, NonZeroU32);
  //aliases!(NiceNonZeroU64, NonZeroU64);
  //aliases!(NiceNonZeroU128, NonZeroU128);
}
