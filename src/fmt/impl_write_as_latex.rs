use super::WriteAsLatex;
use crate::{
    latex_modes::LatexMode,
    latex_flavors::LatexFlavor,
    latex_features::LatexFeatures,
    latex_writer::LatexWriter,
};
use core::fmt::{Error, Write};

macro_rules! impl_for_prim_numeric {
    ($t:ident) => {
        impl<Fl,Fe,M> WriteAsLatex<Fl,Fe,M> for $t
        where
            Fl: LatexFlavor,
            Fe: LatexFeatures,
            M: LatexMode,
        {
            fn write_as_latex<W,NW>(
                &self,
                dest: W,
            ) -> Result<W, Error>
            where
                W: LatexWriter<
                    Flavor = Fl,
                    Features = Fe,
                    Mode = M,
                >,
            {
                let (mut nested_writer, features) = dest.to_raw_parts();
                write!(nested_writer, "{}", self)?;
                Ok(unsafe { W::from_raw_parts(nested_writer, features) })
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
