use super::{
    LatexFormatter, LatexFormatterQuadruple, ZSTQuadruple,
};
use crate::latex_modes::LatexMode;

use core::fmt::{Error, Write};

impl<F,I,IM,OM> LatexFormatter<IM,OM,I> for F
where
    ZSTQuadruple<F,I,IM,OM>: LatexFormatterQuadruple<
        Formatter = F,
        Input = I,
        InitialMode = IM,
        OutputMode = OM
    >,
    IM: LatexMode,
    OM: LatexMode,
{
    fn write_latex<W: Write>(dest: &mut W, input: &I) -> Result<(), Error> {
        ZSTQuadruple::<F,I,IM,OM>::write_latex(dest, input)
    }
}
