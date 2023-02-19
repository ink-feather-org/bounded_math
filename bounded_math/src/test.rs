use core::fmt::Debug;

use crate::{num::IntegerRange, Integer};

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
