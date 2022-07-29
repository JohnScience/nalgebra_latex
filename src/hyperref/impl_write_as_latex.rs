use core::fmt::Write;

use crate::{fmt::WriteAsLatex, latex_modes::MathLatexMode, latex_writer::LatexWriter, latex_features::LatexFeatures};

#[allow(deprecated)]
use super::{SupportedFlavor, Hyperlink};

#[allow(deprecated)]
impl<'a,Fl,Fe,M,NW,IW,OW> WriteAsLatex<Fl,Fe,Fe,M,M,NW,IW,OW> for Hyperlink<'a>
where
    Fl: SupportedFlavor,
    Fe: LatexFeatures,
    M: MathLatexMode,
    NW: Write,
    IW: LatexWriter<
        Flavor = Fl,
        Features = Fe,
        Mode = M,
        NestedWriter = NW,
    >,
    OW: LatexWriter<
        Flavor = Fl,
        Features = Fe,
        Mode = M,
        NestedWriter = NW,
    >,
{
    // TODO: change implementation from the fallback to the actual implementation
    fn write_as_latex(&self, dest: IW) -> Result<OW, core::fmt::Error> {
        let (mut nested_writer, features) = dest.into_raw_parts();
        nested_writer.write_str(r"\textit{")?;
        nested_writer.write_str(&self.reference)?;
        nested_writer.write_str(r"}")?;
        Ok(unsafe { OW::from_raw_parts(nested_writer, features) })
    }
}