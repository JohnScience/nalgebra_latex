use crate::{
    latex_features::LatexFeatures, latex_flavors::LatexFlavor, latex_modes::LatexMode,
    latex_writer::LatexWriter,
};

pub unsafe trait LatexEnvironment<Fl, Fe, M, W, InitW>
where
    Fl: LatexFlavor,
    Fe: LatexFeatures,
    M: LatexMode,
    W: core::fmt::Write,
    InitW: LatexWriter<Flavor = Fl, Features = Fe, NestedWriter = W>,
{
    type InterWriter: LatexWriter<Flavor = Fl, Features = Fe, NestedWriter = W>; // Mode can change

    type OWriter: LatexWriter<Flavor = Fl, Features = Fe, NestedWriter = W>; // Mode can change

    fn write_name(w: &mut W) -> Result<(), core::fmt::Error>;
    fn write_opening_tag(w: InitW) -> Result<Self::InterWriter, core::fmt::Error> {
        let (mut nested_writer, features) = w.into_raw_parts();
        nested_writer.write_str(r"\begin{")?;
        Self::write_name(&mut nested_writer)?;
        nested_writer.write_char('}')?;
        Ok(unsafe { Self::InterWriter::from_raw_parts(nested_writer, features) })
    }
    fn write_closing_tag(w: Self::InterWriter) -> Result<Self::OWriter, core::fmt::Error> {
        let (mut nested_writer, features) = w.into_raw_parts();
        nested_writer.write_str(r"\end{")?;
        Self::write_name(&mut nested_writer)?;
        nested_writer.write_char('}')?;
        Ok(unsafe { Self::OWriter::from_raw_parts(nested_writer, features) })
    }
}
