use crate::InnerType;

pub struct Compare<const A: InnerType, const B: InnerType>;

trait DoCompare {
  const EQ: bool;
  const NE: bool;

  const LT: bool;
  const LE: bool;

  const GT: bool;
  const GE: bool;
}
impl<const A: InnerType, const B: InnerType> DoCompare for Compare<A, B> {
  const EQ: bool = A == B;
  const NE: bool = A != B;
  const LT: bool = A < B;
  const LE: bool = A <= B;
  const GT: bool = A > B;
  const GE: bool = A >= B;
}

pub trait EQ {}
impl<T> EQ for T where T: DoCompare<EQ = true> {}
pub trait NE {}
impl<T> NE for T where T: DoCompare<NE = true> {}
pub trait LT {}
impl<T> LT for T where T: DoCompare<LT = true> {}
pub trait LE {}
impl<T> LE for T where T: DoCompare<LE = true> {}

pub trait GT {}
impl<T> GT for T where T: DoCompare<GT = true> {}

pub trait GE {}
impl<T> GE for T where T: DoCompare<GE = true> {}
