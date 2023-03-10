use core::{fmt::Debug, mem};
use std::{hint::black_box, ops::RangeInclusive};

use test::Bencher;

extern crate test;

use crate::{num::IntegerRange, Integer, NiceU8};

fn assert_in_range<T: IntegerRange + Debug>(val: T, min: &mut bool, max: &mut bool) {
  assert!(T::RANGE.contains(&val.into()), "{val:?}");
  if val.get_value() == *T::RANGE.start() {
    *min = true;
  }
  if val.get_value() == *T::RANGE.end() {
    *max = true;
  }
}

macro_rules! test {
  ($lhs:ty, $rhs:ty) => {
    let mut add_min = false;
    let mut add_max = false;

    let mut sub_min = false;
    let mut sub_max = false;

    let mut mul_min = false;
    let mut mul_max = false;

    let mut div_min = false;
    let mut div_max = false;

    let mut rem_min = false;
    let mut rem_max = false;

    for a in <$lhs>::RANGE {
      for b in <$rhs>::RANGE {
        let a = a.try_to::<Integer<{ <$lhs>::RANGE }>>().unwrap();
        let b = b.try_to::<Integer<{ <$rhs>::RANGE }>>().unwrap();
        assert_in_range(a + b, &mut add_min, &mut add_max);
        assert_in_range(a - b, &mut sub_min, &mut sub_max);
        assert_in_range(a * b, &mut mul_min, &mut mul_max);
        if b.get_value() != 0 {
          assert_in_range(a / b, &mut div_min, &mut div_max);
        }
        if b.get_value() != 0 {
          assert_in_range(a % b, &mut rem_min, &mut rem_max)
        }
      }
    }

    assert!(add_min);
    assert!(add_max);

    assert!(sub_min);
    assert!(sub_max);

    assert!(mul_min);
    assert!(mul_max);

    if <$rhs>::RANGE != (0..=0) {
      assert!(div_min);
      assert!(div_max);

      assert!(div_min);
      assert!(div_max);
    }
  };
}

macro_rules! generate_tests {
  (($start1:literal, $end1:literal) $(($start:literal, $end:literal))+) => {
    generate_tests!(($start1, $end1));
    $(
      test!(Integer<{$start1..=$end1}>, Integer<{$start..=$end}>);
    )?
    generate_tests!($(($start, $end))+);
  };
  (($start1:literal, $end1:literal)) => {
    test!(Integer<{$start1..=$end1}>, Integer<{$start1..=$end1}>);
  };
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn test_gen() {
  generate_tests!((3, 64)(-3, 64)(-64, 3)(-64, -3)(2, 64)(-2, 64)(-64, 2)(
    -64, -2
  )(-1, 64)(-64, 64)(-64, 1)(-64, -1)(0, 64)(0, 64)(-64, 0));
}
#[test]
fn test_only_pos_only_neg() {
  test!(Integer<{ 1..=64 }>, Integer<{ -64..=-1 }>);
}
#[test]
fn test_only_neg_only_pos() {
  test!(Integer<{ -64..=-1 }>, Integer<{ 1..=64 }>);
}
#[test]
fn test_only_pos_only_neg_two() {
  test!(Integer<{ 2..=64 }>, Integer<{ -64..=-2 }>);
}
#[test]
fn test_only_neg_only_pos_two() {
  test!(Integer<{ -64..=-2 }>, Integer<{ 2..=64 }>);
}
#[test]
fn test_only_pos_only_neg_three() {
  test!(Integer<{ 3..=64 }>, Integer<{ -64..=-3 }>);
}
#[test]
fn test_only_neg_only_pos_three() {
  test!(Integer<{ -64..=-3 }>, Integer<{ 3..=64 }>);
}
#[test]
fn test_i8_i8() {
  test!(i8, i8);
}
#[test]
fn test_i8_u8() {
  test!(i8, u8);
}
#[test]
fn test_u8_i8() {
  test!(u8, i8);
}
#[test]
fn test_u8_u8() {
  test!(u8, u8);
}

#[test]
fn test_size() {
  assert_eq!(mem::size_of::<Integer<{ -1..=1 }>>(), 1);
  assert_eq!(mem::size_of::<Integer<{ -128..=127 }>>(), 1);

  assert_eq!(mem::size_of::<Integer<{ -128..=128 }>>(), 2);
  assert_eq!(
    mem::size_of::<Integer<{ (i16::MIN.into())..=(i16::MAX.into()) }>>(),
    2
  );
  assert_eq!(
    mem::size_of::<Integer<{ (i32::MIN.into())..=(i32::MAX.into()) }>>(),
    4
  );
  assert_eq!(
    mem::size_of::<Integer<{ (i64::MIN.into())..=(i64::MAX.into()) }>>(),
    8
  );
  assert_eq!(mem::size_of::<Integer<{ i128::MIN..=i128::MAX }>>(), 16);

  assert_eq!(
    mem::size_of::<Integer<{ (i16::MIN.into())..=(i128::from(i16::MIN) + i128::from(u8::MAX)) }>>(),
    1
  );

  assert_eq!(
    mem::size_of::<Integer<{ (i16::MIN.into())..=(i16::MIN.into()) }>>(),
    0
  );
}

#[test]
fn test_bound() {
  const A: i128 = 123_456;
  const B: i128 = 1_234_567;
  const C: i128 = A + B;

  let a = Integer::<{ A..=(A + 1) }>::new::<A>();
  let b = Integer::<{ B..=(B + 1) }>::new::<B>();
  let sum = a + b;

  let expected = Integer::<{ C..=C }>::new_exact();
  assert!(sum == expected);
}
#[test]
fn test_from() {
  assert!(NiceU8::try_from(256).is_err());
  assert!(NiceU8::try_from(255).is_ok());
}

macro_rules! gen_bench {
  ($start:literal..=$end:literal, $prim:ty, $name_prim:ident, $name_integer:ident) => {
    #[bench]
    #[cfg(not(miri))]
    fn $name_prim(b: &mut Bencher) {
      const RANGE: RangeInclusive<i128> = $start..=$end;
      b.iter(|| {
        for a in RANGE {
          for b in RANGE {
            let a = <$prim>::try_from(a).unwrap();
            let b = <$prim>::try_from(b).unwrap();
            black_box(a.wrapping_add(b));
          }
        }
      });
    }

    #[bench]
    #[cfg(not(miri))]
    fn $name_integer(b: &mut Bencher) {
      const RANGE: RangeInclusive<i128> = $start..=$end;
      b.iter(|| {
        for a in RANGE {
          for b in RANGE {
            let a = Integer::<RANGE>::try_from(a).unwrap();
            let b = Integer::<RANGE>::try_from(b).unwrap();
            black_box(a + b);
          }
        }
      });
    }
  };
}
gen_bench! {0..=255, u8, bench_u8, bench_u8_integer}
gen_bench! {-1..=254, i16, bench_n1_254_i16, bench_n1_254_integer}

gen_bench! {-1200..=-1000, i16, bench_n1200_n1000_i16, bench_n1200_n1000_integer}
