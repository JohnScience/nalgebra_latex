use crate::latex_writer::LatexWriter;

pub trait LatexEnvironment<Fl, Fe, W, InitW>
where
    W: core::fmt::Write,
    InitW: LatexWriter<Flavor = Fl, Features = Fe, NestedWriter = W>,
{
    type InterWriter: LatexWriter<Flavor = Fl, Features = Fe, NestedWriter = W>; // Mode can change

    type OWriter: LatexWriter<Flavor = Fl, Features = Fe, NestedWriter = W>; // Mode can change

    fn write_name(w: &mut W) -> Result<(), core::fmt::Error>;
    fn write_opening_tag(w: InitW) -> Result<Self::InterWriter, core::fmt::Error>;
    unsafe fn write_closing_tag_unchecked(
        w: W,
        features: Fe,
    ) -> Result<Self::OWriter, core::fmt::Error>;
    #[inline(always)]
    fn write_closing_tag(w: Self::InterWriter) -> Result<Self::OWriter, core::fmt::Error> {
        let (nested_writer, features) = w.into_raw_parts();
        unsafe { Self::write_closing_tag_unchecked(nested_writer, features) }
    }
}
