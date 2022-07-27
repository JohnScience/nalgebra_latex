use super::ConsumingWriteAsLatex;
use crate::{
    latex_features::LatexFeatures, latex_flavors::LatexFlavor, latex_modes::LatexMode,
    latex_writer::LatexWriter,
};
use core::fmt::{Error, Write};

macro_rules! impl_for_prim_numeric {
    ($t:ident) => {
        impl<Fl, Fe, M, NestedWriter, W> ConsumingWriteAsLatex<Fl, Fe, Fe, M, M, NestedWriter, W, W>
            for $t
        where
            Fl: LatexFlavor,
            Fe: LatexFeatures,
            M: LatexMode,
            NestedWriter: Write,
            W: LatexWriter<Flavor = Fl, Features = Fe, Mode = M, NestedWriter = NestedWriter>,
        {
            fn consuming_write_as_latex(self, dest: W) -> Result<W, Error> {
                let (mut nested_writer, features) = dest.into_raw_parts();
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

impl<
        Flavor,
        InitialFeatures,
        ConsequentFeatures,
        InitialLatexMode,
        ConsequentLatexMode,
        NestedWriter,
        InitalWriter,
        OutputWriter,
    >
    ConsumingWriteAsLatex<
        Flavor,
        InitialFeatures,
        ConsequentFeatures,
        InitialLatexMode,
        ConsequentLatexMode,
        NestedWriter,
        InitalWriter,
        OutputWriter,
    > for fn(InitalWriter) -> Result<OutputWriter, Error>
where
    Flavor: LatexFlavor,
    InitialFeatures: LatexFeatures,
    ConsequentFeatures: LatexFeatures,
    InitialLatexMode: LatexMode,
    ConsequentLatexMode: LatexMode,
    NestedWriter: Write,
    InitalWriter: LatexWriter<
        Flavor = Flavor,
        Features = InitialFeatures,
        Mode = InitialLatexMode,
        NestedWriter = NestedWriter,
    >,
    OutputWriter: LatexWriter<
        Flavor = Flavor,
        Features = ConsequentFeatures,
        Mode = ConsequentLatexMode,
        NestedWriter = NestedWriter,
    >,
{
    fn consuming_write_as_latex(self, dest: InitalWriter) -> Result<OutputWriter, Error> {
        (self)(dest)
    }
}
