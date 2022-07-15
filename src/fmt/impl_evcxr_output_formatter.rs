use crate::latex_modes::LatexMode;

use super::{EvcxrOutputFormatter, LatexFormatter};

impl<M,F,IM,OM,I> EvcxrOutputFormatter<M,I> for F
where
    M: mime_typed::MimeStrExt,
    IM: LatexMode,
    OM: LatexMode,
    F: LatexFormatter<IM,OM,I>
{
    fn write_evcxr_output<W: core::fmt::Write>(&self, dest: &mut W, input: &I) -> Result<(), core::fmt::Error>
    {
        todo!()
    }
}
