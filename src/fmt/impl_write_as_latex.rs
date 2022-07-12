use super::WriteAsLatex;
use crate::latex_modes::LatexMode;
use core::fmt::Display;
use just_prim::PrimNum;

impl<M, T> WriteAsLatex<M> for T
where
    M: LatexMode,
    T: PrimNum + Display,
{
    fn write_as_latex<W>(&self, writer: &mut W) -> Result<(), core::fmt::Error>
    where
        W: core::fmt::Write,
    {
        write!(writer, "{}", self)
    }
}
