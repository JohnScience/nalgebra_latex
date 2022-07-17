use mime_typed::evcxr_support::TextMarkdownUtf8;
use crate::latex_modes::{InnerParagraphMode, DisplayMathMode};

use super::{EvcxrOutputFormatter, LatexFormatter};

impl<F,I> EvcxrOutputFormatter<TextMarkdownUtf8,I> for F
where
    F: LatexFormatter<InnerParagraphMode,DisplayMathMode,I>
{
    fn write_evcxr_output<W: core::fmt::Write>(&self, dest: &mut W, input: &I) -> Result<(), core::fmt::Error>
    {
        F::write_latex(dest, input)
    }
}
