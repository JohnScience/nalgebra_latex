use crate::{
    latex_flavors::LatexFlavor,
    latex_writer::LatexWriter, latex_features::LatexFeatures, latex_modes::LatexMode,
};

mod impl_write_as_latex;

pub trait LatexFormatter<
    Flavor,
    InitialFeatures,
    ConsequentFeatures,
    InitialMode,
    ConsequentMode,
    I,
>
    where
        Flavor: LatexFlavor,
        InitialFeatures: LatexFeatures,
        ConsequentFeatures: LatexFeatures,
        InitialMode: LatexMode,
        ConsequentMode: LatexMode,
{
    fn write<IW,OW>(dest: IW, input: &I) -> Result<OW, core::fmt::Error>
    where
        IW: LatexWriter<
            Flavor = Flavor,
            Features = InitialFeatures,
            Mode = InitialMode,
        >,
        OW: LatexWriter<
            Flavor = Flavor,
            Features = ConsequentFeatures,
            Mode = ConsequentMode,
        >;
}

pub trait WriteAsLatex<Fl,Fe,M>
where
    M: LatexMode
{
    fn write_as_latex<W,NW>(&self, dest: W) -> Result<W,core::fmt::Error>
    where
        NW: core::fmt::Write,
        W: LatexWriter<
            Flavor = Fl,
            Features = Fe,
            Mode = M,
            NestedWriter = NW,
        >;
}

