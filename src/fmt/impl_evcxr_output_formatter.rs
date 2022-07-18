use crate::latex_modes::{DisplayMathMode, InnerParagraphMode};
use mime_typed::{evcxr_support::TextMarkdown, MimeStrExt};

use super::{EvcxrOutputFormatter, LatexFormatter};

impl<F, I> EvcxrOutputFormatter<TextMarkdown, I> for F
where
    F: LatexFormatter<InnerParagraphMode, DisplayMathMode, I>,
{
    fn write_evcxr_output<W: core::fmt::Write>(
        dest: &mut W,
        input: &I,
    ) -> Result<(), core::fmt::Error> {
        writeln!(dest, "EVCXR_BEGIN_CONTENT {}", TextMarkdown::MIME_STR)?;
        F::write_latex(dest, input)?;
        dest.write_str("\nEVCXR_END_CONTENT\n")
    }
}
