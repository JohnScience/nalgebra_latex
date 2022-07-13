use super::WriteAsLatex;
use crate::latex_modes::LatexMode;

macro_rules! impl_for_prim_numeric {
    ($t:ident) => {
        impl<M: LatexMode> WriteAsLatex<M> for $t {
            fn write_as_latex<W: core::fmt::Write>(&self, dest: &mut W) -> Result<(), core::fmt::Error> {
                write!(dest, "{}", self)
            }
        }
    };
}

impl_for_prim_numeric!(u8);
impl_for_prim_numeric!(u16);
impl_for_prim_numeric!(u32);
impl_for_prim_numeric!(u64);
impl_for_prim_numeric!(u128);
impl_for_prim_numeric!(usize);
impl_for_prim_numeric!(i8);
impl_for_prim_numeric!(i16);
impl_for_prim_numeric!(i32);
impl_for_prim_numeric!(i64);
impl_for_prim_numeric!(i128);
impl_for_prim_numeric!(isize);
impl_for_prim_numeric!(f32);
impl_for_prim_numeric!(f64);