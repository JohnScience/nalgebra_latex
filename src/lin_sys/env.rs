use crate::{env::LatexEnvironment, latex_flavors::LatexFlavor, latex_features::LatexFeatures, latex_modes::MathLatexMode, latex_writer::LatexWriter};

pub struct CasesEnvironment;

unsafe impl<Fl,Fe,M,W,InitW> LatexEnvironment<Fl,Fe,M,W,InitW> for CasesEnvironment
where
    Fl: LatexFlavor,
    Fe: LatexFeatures,
    M: MathLatexMode,
    W: core::fmt::Write,
    InitW: LatexWriter<Flavor = Fl, Features = Fe, Mode = M, NestedWriter = W>,
{
    type InterWriter = InitW;
    type OWriter = InitW;
    fn write_name(w: &mut W) -> Result<(), core::fmt::Error> {
        w.write_str("cases")
    }
}