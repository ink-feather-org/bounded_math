use crate::BigUInt;

#[test]
fn test_rt() {
  let v1 = BigUInt::from(5u16);
  let v2 = BigUInt::from(10u64);

  assert!(v1 == 5u8.into());
  assert!(v2 == 10u8.into());

  let res = v1 + v2;
  assert!(res == 15u8.into());
}
#[test]
fn test_ct() {
  const {
    let v1 = BigUInt::from(5u16);
    let v2 = BigUInt::from(10u64);

    let res = v1 + v2;
    assert!(res == 15u8.into())
  }
}

#[test]
fn test_rt_u8() {
  let v1 = BigUInt::from(200u8);
  let v2 = BigUInt::from(128u8);

  let res = v1 + v2;
  assert!(res == 228u16.into());
}
#[test]
fn test_ct_u8() {
  let res = {
    let v1 = BigUInt::from(200u8);
    let v2 = BigUInt::from(128u8);

    (v1 + v2).leak_to_rt()
  };
  assert!(res == 228u16.into());
}
