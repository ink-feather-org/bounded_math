use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use crate::{cmp::*, InnerType};

pub struct Integer<const MIN_VAL: InnerType, const MAX_VAL: InnerType>
where
    Compare<MIN_VAL, MAX_VAL>: LE,
{
    val: InnerType,
}

impl<const MIN_VAL: InnerType, const MAX_VAL: InnerType> Debug for Integer<MIN_VAL, MAX_VAL>
where
    Compare<MIN_VAL, MAX_VAL>: LE,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        writeln!(f, "Integer<{} to {}>: {}", MIN_VAL, MAX_VAL, self.val)
    }
}

impl<const MIN_VAL: InnerType, const MAX_VAL: InnerType> Integer<MIN_VAL, MAX_VAL>
where
    Compare<MIN_VAL, MAX_VAL>: LE,
{
    pub const fn new<const VALUE: InnerType>() -> Self
    where
        Compare<MIN_VAL, VALUE>: LE,
        Compare<VALUE, MAX_VAL>: LE,
    {
        Self { val: VALUE }
    }

    pub fn try_change_bounds<const OUTPUT_MIN: InnerType, const OUTPUT_MAX: InnerType>(
        &self,
    ) -> Option<Integer<OUTPUT_MIN, OUTPUT_MAX>>
    where
        Compare<OUTPUT_MIN, OUTPUT_MAX>: LE,
    {
        if OUTPUT_MIN <= MIN_VAL && OUTPUT_MAX >= MAX_VAL
            || (OUTPUT_MIN..=OUTPUT_MAX).contains(&self.val)
        {
            Some(Integer::<OUTPUT_MIN, OUTPUT_MAX> { val: self.val })
        } else {
            None
        }
    }

    pub const fn grow_bounds<const OUTPUT_MIN: InnerType, const OUTPUT_MAX: InnerType>(
        &self,
    ) -> Integer<OUTPUT_MIN, OUTPUT_MAX>
    where
        Compare<OUTPUT_MIN, OUTPUT_MAX>: LE,
        Compare<OUTPUT_MIN, MIN_VAL>: LE,
        Compare<OUTPUT_MAX, MAX_VAL>: GE,
    {
        Integer::<OUTPUT_MIN, OUTPUT_MAX> { val: self.val }
    }
}
impl<const VALUE: InnerType> Integer<VALUE, VALUE>
where
    Compare<VALUE, VALUE>: LE,
{
    pub const fn new_exact() -> Self {
        Self { val: VALUE }
    }
}
//impl<
//        const SRC_MIN_VAL: InnerType,
//        const SRC_MAX_VAL: InnerType,
//        const DST_MIN_VAL: InnerType,
//        const DST_MAX_VAL: InnerType,
//    > From<Integer<SRC_MIN_VAL, SRC_MAX_VAL>> for Integer<DST_MIN_VAL, DST_MAX_VAL>
//where
//    Compare<SRC_MIN_VAL, SRC_MAX_VAL>: LE,
//    Compare<DST_MIN_VAL, DST_MAX_VAL>: LE,
//    Compare<SRC_MIN_VAL, DST_MIN_VAL>: LT,
//{
//    fn from(value: Integer<SRC_MIN_VAL, SRC_MAX_VAL>) -> Self {
//        todo!()
//    }
//}

//impl<
//        const SRC_MIN_VAL: InnerType,
//        const SRC_MAX_VAL: InnerType,
//        const DST_MIN_VAL: InnerType,
//        const DST_MAX_VAL: InnerType,
//    > TryFrom<Integer<SRC_MIN_VAL, SRC_MAX_VAL>> for Integer<DST_MIN_VAL, DST_MAX_VAL>
//where
//    Compare<SRC_MIN_VAL, SRC_MAX_VAL>: LE,
//    Compare<DST_MIN_VAL, DST_MAX_VAL>: LE,
//    Compare<SRC_MIN_VAL, DST_MIN_VAL>: LT,
//    Compare<SRC_MAX_VAL, DST_MAX_VAL>: GT,
//{
//    type Error = ();
//
//    fn try_from(value: Integer<SRC_MIN_VAL, SRC_MAX_VAL>) -> Result<Self, Self::Error> {
//        todo!()
//    }
//}

macro_rules! impl_op {
    ($op_tr:ident, $op_fn_name:ident, $op_symb:tt) => {
        impl<
                const LHS_MIN_VAL: InnerType,
                const LHS_MAX_VAL: InnerType,
                const RHS_MIN_VAL: InnerType,
                const RHS_MAX_VAL: InnerType,
            > $op_tr<Integer<LHS_MIN_VAL, LHS_MAX_VAL>> for Integer<RHS_MIN_VAL, RHS_MAX_VAL>
        where
        Compare<LHS_MIN_VAL, LHS_MAX_VAL>: LE,
        Compare<RHS_MIN_VAL, RHS_MAX_VAL>: LE,
        Compare<{ LHS_MIN_VAL $op_symb RHS_MIN_VAL }, { LHS_MAX_VAL $op_symb RHS_MAX_VAL }>: LE,
        {
            type Output = Integer<{ LHS_MIN_VAL $op_symb RHS_MIN_VAL }, { LHS_MAX_VAL $op_symb RHS_MAX_VAL }>;

            fn $op_fn_name(self, rhs: Integer<LHS_MIN_VAL, LHS_MAX_VAL>) -> Self::Output {
                Integer {
                    val: self.val $op_symb rhs.val,
                }
            }
        }
    };
}
impl_op! {Add, add, +}
impl_op! {Mul, mul, *}

macro_rules! aliases {
    ($nice_name:ident, $base_type:ty) => {
        pub type $nice_name =
            Integer<{ <$base_type>::MIN as InnerType }, { <$base_type>::MAX as InnerType }>;
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
//aliases!(NiceI128, i128);

macro_rules! impl_try_from {
    ($from_type:ty) => {
        impl<const MIN_VAL: InnerType, const MAX_VAL: InnerType> From<$from_type>
            for Integer<MIN_VAL, MAX_VAL>
        where
            Compare<MIN_VAL, MAX_VAL>: LE,
            Compare<MIN_VAL, { <$from_type>::MIN as InnerType }>: LE,
            Compare<MAX_VAL, { <$from_type>::MAX as InnerType }>: GE,
        {
            fn from(from_val: $from_type) -> Self {
                Integer {
                    val: from_val as InnerType,
                }
            }
        }

        /*impl<const MIN_VAL: InnerType, const MAX_VAL: InnerType> TryFrom<Integer<MIN_VAL, MAX_VAL>>
            for $from_type
        where
            Compare<MIN_VAL, MAX_VAL>: LT,
        {
            type Error = ();
            fn try_from(from_val: Integer<MIN_VAL, MAX_VAL>) -> Result<Self, Self::Error> {
                if (from_val.val >= <$from_type>::MIN as InnerType
                    && from_val.val <= <$from_type>::MAX as InnerType)
                {
                    Ok(from_val.val as $from_type)
                } else {
                    Err(())
                }
            }
        }*/
    };
}

impl_try_from!(u8);
impl_try_from!(u16);
impl_try_from!(u32);
impl_try_from!(u64);
//impl_try_from!(u128);

impl_try_from!(i8);
impl_try_from!(i16);
impl_try_from!(i32);
impl_try_from!(i64);
//impl_try_from!(i128);
