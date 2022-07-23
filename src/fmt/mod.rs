use crate::{
    latex_features::LatexFeatures, latex_flavors::LatexFlavor, latex_modes::LatexMode,
    latex_writer::LatexWriter,
};

mod impl_write_as_latex;
mod impl_partial_endofunctional_write_as_latex;

pub trait LatexFormatter<
    Flavor,
    InitialFeatures,
    ConsequentFeatures,
    InitialMode,
    ConsequentMode,
    I,
> where
    Flavor: LatexFlavor,
    InitialFeatures: LatexFeatures,
    ConsequentFeatures: LatexFeatures,
    InitialMode: LatexMode,
    ConsequentMode: LatexMode,
{
    fn write<IW, OW>(dest: IW, input: &I) -> Result<OW, core::fmt::Error>
    where
        IW: LatexWriter<
            Flavor = Flavor,
            Features = InitialFeatures,
            Mode = InitialMode,
            NestedWriter = OW::NestedWriter,
        >,
        OW: LatexWriter<Flavor = Flavor, Features = ConsequentFeatures, Mode = ConsequentMode>;
}

pub trait WriteAsLatex<Fl,InitFe,ConseqFe,InitM,ConseqM, NestedWriter,IW,OW>
where
    Fl: LatexFlavor,
    InitFe: LatexFeatures,
    ConseqFe: LatexFeatures,
    InitM: LatexMode,
    ConseqM: LatexMode,
    NestedWriter: core::fmt::Write,
    IW: LatexWriter<Flavor = Fl, Features = InitFe, Mode = InitM, NestedWriter = NestedWriter>,
    OW: LatexWriter<Flavor = Fl, Features = ConseqFe, Mode = ConseqM, NestedWriter = NestedWriter>,
{
    fn write_as_latex(&self, dest: IW) -> Result<OW, core::fmt::Error>;
}

// The function is endofunctional in the second argument in a sense that
// partial!(partial_endofunctional_write_as_latex ; input, _ ) maps a writer
// to another writer of the same type [though, presumably with modified state]
// and does not have totality due to formatting errors.
pub trait PartialEndofunctionalWriteAsLatex<Fl, Fe, M>
where
    Fl: LatexFlavor,
    Fe: LatexFeatures,
    M: LatexMode,
{
    fn partial_endofunctional_write_as_latex<W, NW>(&self, dest: W) -> Result<W, core::fmt::Error>
    where
        NW: core::fmt::Write,
        W: LatexWriter<Flavor = Fl, Features = Fe, Mode = M, NestedWriter = NW>;
}
