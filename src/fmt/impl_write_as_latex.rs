use super::{WriteAsLatex, labels::{CountersLabel, Label, SupportedFlavor}};
use crate::{
    latex_features::LatexFeatures, latex_flavors::LatexFlavor, latex_modes::{LatexMode, InlineMathMode},
    latex_writer::LatexWriter,
};
use core::fmt::{Error, Write};

macro_rules! impl_for_prim_numeric {
    ($t:ident) => {
        impl<Fl, Fe, M, NestedWriter, W> WriteAsLatex<Fl, Fe, Fe, M, M, NestedWriter, W, W> for $t
        where
            Fl: LatexFlavor,
            Fe: LatexFeatures,
            M: LatexMode,
            NestedWriter: Write,
            W: LatexWriter<Flavor = Fl, Features = Fe, Mode = M, NestedWriter = NestedWriter>,
        {
            fn write_as_latex(&self, dest: W) -> Result<W, Error> {
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
    WriteAsLatex<
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
    fn write_as_latex(&self, dest: InitalWriter) -> Result<OutputWriter, Error> {
        (self)(dest)
    }
}

impl<Fl,Fe,W,IW,OW> WriteAsLatex<Fl, Fe, Fe, InlineMathMode, InlineMathMode, W, IW, OW>
    for CountersLabel
where
    Fl: SupportedFlavor,
    Fe: LatexFeatures,
    W: Write,
    IW: LatexWriter<
        Flavor = Fl,
        Features = Fe,
        Mode = InlineMathMode,
        NestedWriter = W,
    >,
    OW: LatexWriter<
        Flavor = Fl,
        Features = Fe,
        Mode = InlineMathMode,
        NestedWriter = W,
    >,
{
    fn write_as_latex(&self, dest: IW) -> Result<OW, Error> {
        let (mut nested_writer, features) = dest.into_raw_parts();
        let is_referencable =  Fl::is_referencable(self);
        nested_writer.write_str(if is_referencable { r"/eqref{" } else { "(" } )?;
        self.write_name(&mut nested_writer)?;
        nested_writer.write_char(if is_referencable { '}' } else { ')' } )?;
        Ok(unsafe { OW::from_raw_parts(nested_writer, features) })
    }
}
