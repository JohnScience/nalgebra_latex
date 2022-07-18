use crate::latex_modes::{DisplayMathMode, InnerParagraphMode};
use mime_typed::evcxr_support::TextMarkdownUtf8;

use super::{EvcxrOutputFormatter, LatexFormatter};

impl<F, I> EvcxrOutputFormatter<TextMarkdownUtf8, I> for F
where
    F: LatexFormatter<InnerParagraphMode, DisplayMathMode, I>,
{
    fn write_evcxr_output<W: core::fmt::Write>(
        dest: &mut W,
        input: &I,
    ) -> Result<(), core::fmt::Error> {
        F::write_latex(dest, input)
    }
}
