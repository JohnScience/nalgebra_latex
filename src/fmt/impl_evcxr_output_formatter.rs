use mime_typed::evcxr_support::TextMarkdownUtf8;
use crate::latex_modes::{InnerParagraphMode, DisplayMathMode};

use super::{EvcxrOutputFormatter, LatexFormatterQuadruple};

impl<F,I> EvcxrOutputFormatter<TextMarkdownUtf8,I> for F
where
    (F,I,InnerParagraphMode,DisplayMathMode): LatexFormatterQuadruple<
        Formatter = F,
        Input = I,
        InitialMode = InnerParagraphMode,
        OutputMode = DisplayMathMode
    >,
{
    fn write_evcxr_output<W: core::fmt::Write>(&self, dest: &mut W, input: &I) -> Result<(), core::fmt::Error>
    {
        <(F,I,InnerParagraphMode,DisplayMathMode)>::write_latex(dest, input)
    }
}
