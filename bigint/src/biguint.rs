use core::{
  cmp::Ordering,
  error::Error,
  fmt,
  fmt::Display,
  marker::Destruct,
  ops::Div,
  ops::{Add, Mul, Shl, Shr, Sub},
};

use const_box::ConstBox;

pub struct BigUInt<T: AsRef<[u8]>>(T);

impl BigUInt<ConstBox<[u8]>> {
  fn leak_to_rt(self) -> BigUInt<&'static [u8]> {
    BigUInt(self.0.leak_to_rt())
  }
}

// TODO ceil_log2

const fn drop<T: ~const Destruct>(_: T) {}
/*
impl const Add<BigUInt> for BigUInt {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let len_no_ov = self.0.len().max(rhs.0.len());
    let mut out = ConstBox::new_uninit_slice(len_no_ov);

    let mut last_was_overflow = false;
    let mut i = 0;
    while i < len_no_ov {
      let val = *self.0.get(i).unwrap_or(&0);
      let (val, overflow1) = val.overflowing_add(*rhs.0.get(i).unwrap_or(&0));

      let (val, overflow2) = if last_was_overflow {
        val.overflowing_add(1)
      } else {
        (val, false)
      };

      last_was_overflow = overflow1 || overflow2;

      out[i].write(val);
      i += 1;
    }

    let out = unsafe { out.assume_init() };

    let out = if last_was_overflow {
      let mut new_out = ConstBox::new_uninit_slice(len_no_ov + 1);
      let mut i = 0;
      while i < len_no_ov {
        new_out[i].write(out[i]);
        i += 1;
      }
      new_out[i].write(1);

      drop(out);

      unsafe { new_out.assume_init() }
    } else {
      out
    };
    Self(out)
  }
}
impl const Sub<BigUInt> for BigUInt {
  type Output = Self;

  fn sub(self, _rhs: Self) -> Self::Output {
    todo!()
  }
}

impl const Mul<BigUInt> for BigUInt {
  type Output = Self;

  fn mul(self, _rhs: Self) -> Self::Output {
    todo!()
  }
}
impl const Div<BigUInt> for BigUInt {
  type Output = Self;

  fn div(self, _rhs: Self) -> Self::Output {
    todo!()
  }
}

impl const Shl<usize> for BigUInt {
  type Output = Self;

  fn shl(self, _rhs: usize) -> Self::Output {
    todo!()
  }
}
impl const Shr<usize> for BigUInt {
  type Output = Self;

  fn shr(self, _rhs: usize) -> Self::Output {
    todo!()
  }
}

impl const PartialEq for BigUInt {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(&other).is_eq()
  }
}
impl Eq for BigUInt {}

impl const PartialOrd for BigUInt {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}
impl const Ord for BigUInt {
  fn cmp(&self, other: &Self) -> Ordering {
    let mut i = self.0.len() - 1;
    while i != 0 {
      let lhs = self.0.get(i).unwrap_or(&0);
      let rhs = other.0.get(i).unwrap_or(&0);
      match lhs.cmp(rhs) {
        Ordering::Equal => {},
        other => return other,
      }
      i -= 1;
    }
    return Ordering::Equal;
  }
}
 */
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TryFromBigUIntError;

impl Display for TryFromBigUIntError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Tried to convert an BigUInt to a ")
  }
}
impl Error for TryFromBigUIntError {}

macro_rules! from_impls {
    ($(($prim:ty, $bytes:literal)),+) => {$(
      impl const From<$prim> for BigUInt {
        fn from(value: $prim) -> Self {
          Self(ConstBox::new(value.to_le_bytes()))
        }
      }
      impl const From<&$prim> for BigUInt {
        fn from(value: &$prim) -> Self {
          Self(ConstBox::new(value.to_le_bytes()))
        }
      }
      impl const TryFrom<BigUInt> for $prim {
        type Error = TryFromBigUIntError;

        fn try_from(value: BigUInt) -> Result<Self, Self::Error> {
          if value.0.len() <= $bytes {
            let mut array = [0; $bytes];
            let mut i = 0;
            while i < value.0.len() {
              array[i] = value.0[i];
              i += 1;
            }
            Ok(Self::from_le_bytes(array))
          } else {

            Err(TryFromBigUIntError)
          }
        }
      }
    )*};
  }

from_impls! {(u8, 1), (u16, 2), (u32, 4), (u64, 8), (u128, 16)}
