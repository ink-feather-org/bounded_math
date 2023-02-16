use core::{fmt::Debug, ops::RangeInclusive};
use std::marker::Destruct;

use crate::InnerType;
pub trait RangeIsEmpty<const RANGE: RangeInclusive<InnerType>> {
  const RET: bool;
}
impl<const RANGE: RangeInclusive<InnerType>> RangeIsEmpty<RANGE> for () {
  const RET: bool = RANGE.is_empty();
}

#[derive(Clone, Copy)]
pub struct Integer<const RANGE: RangeInclusive<InnerType>>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  pub(crate) val: InnerType,
}

trait ContainsRet<const VALUE: InnerType> {
  const RET: bool;
}

impl<const RANGE: RangeInclusive<InnerType>, const VALUE: InnerType> ContainsRet<VALUE>
  for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  const RET: bool = RANGE.contains(&VALUE);
}

pub trait ValInRange<const VALUE: InnerType> {}
impl<const RANGE: RangeInclusive<InnerType>, const VALUE: InnerType> ValInRange<VALUE>
  for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
  Self: ContainsRet<VALUE, RET = true>,
{
}
pub trait RangeInRange<const CONTAINED_RANGE: RangeInclusive<InnerType>> {
  const CONTAINED: bool;
}
impl<const RANGE: RangeInclusive<InnerType>, const CONTAINED_RANGE: RangeInclusive<InnerType>>
  RangeInRange<CONTAINED_RANGE> for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  const CONTAINED: bool =
    RANGE.contains(CONTAINED_RANGE.start()) && RANGE.contains(CONTAINED_RANGE.end());
}

impl<const RANGE: RangeInclusive<InnerType>> Debug for Integer<RANGE>
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
impl<const RANGE: RangeInclusive<InnerType>> IsExact for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  const EXACT: bool = RANGE.start() == RANGE.end();
}

impl<const RANGE: RangeInclusive<InnerType>> Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  pub const fn new<const VALUE: InnerType>() -> Self
  where
    Self: ValInRange<VALUE>,
  {
    Self { val: VALUE }
  }
  pub const fn new_exact() -> Self
  where
    Self: IsExact<EXACT = true>,
  {
    Self {
      val: *RANGE.start(),
    }
  }
}
trait RangeNotEmpty {}

#[const_trait]
pub trait IntegerRange: Copy + ~const Into<InnerType> {
  const RANGE: RangeInclusive<InnerType>;

  fn get_value(self) -> InnerType;

  fn to_integer(self) -> Integer<{ Self::RANGE }>
  where
    (): RangeIsEmpty<{ Self::RANGE }, RET = false>,
  {
    let Ok(ret) = self.get_value().try_into() else {
      unreachable!()
    };
    ret
  }

  fn to<T: ~const IntegerRange + ~const TryFrom<InnerType>>(self) -> T
  where
    (): RangeIsEmpty<{ Self::RANGE }, RET = false>,
    (): RangeIsEmpty<{ T::RANGE }, RET = false>,
    Integer<{ T::RANGE }>: RangeInRange<{ Self::RANGE }, CONTAINED = true>,
    Result<T, T::Error>: ~const Destruct,
  {
    let Ok(ret) = self.get_value().try_into() else {
        unreachable!()
    };
    ret
  }
  fn try_to<T: ~const IntegerRange + ~const From<InnerType>>(self) -> Option<T>
  where
    (): RangeIsEmpty<{ Self::RANGE }, RET = false>,
    (): RangeIsEmpty<{ T::RANGE }, RET = false>,
  {
    if Self::RANGE.contains(&Self::RANGE.start()) && Self::RANGE.contains(&Self::RANGE.end())
      || Self::RANGE.contains(&self.into())
    {
      Some(self.get_value().into())
    } else {
      None
    }
  }
}

impl<const RANGE_GEN: RangeInclusive<InnerType>> const IntegerRange for Integer<RANGE_GEN>
where
  (): RangeIsEmpty<RANGE_GEN, RET = false>,
{
  const RANGE: RangeInclusive<InnerType> = RANGE_GEN;

  fn get_value(self) -> InnerType {
    self.val
  }
}

impl<const RANGE: RangeInclusive<InnerType>> const From<Integer<RANGE>> for InnerType
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  fn from(value: Integer<RANGE>) -> Self {
    value.val
  }
}
impl<const RANGE: RangeInclusive<InnerType>> const From<InnerType> for Integer<RANGE>
where
  (): RangeIsEmpty<RANGE, RET = false>,
{
  fn from(value: InnerType) -> Self {
    Self { val: value }
  }
}
pub mod aliases {
  //use core::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64};

  use super::*;
  macro_rules! aliases {
    ($nice_name:ident, $base_type:ty) => {
      impl const IntegerRange for $base_type {
        const RANGE: RangeInclusive<InnerType> =
          <$base_type>::MIN as InnerType..=<$base_type>::MAX as InnerType;
        fn get_value(self) -> InnerType {
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
