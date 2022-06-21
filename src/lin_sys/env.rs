use crate::env::LatexEnvironment;

/// Representation of `cases` [LaTeX] [environment].
/// 
/// For examples of usage, see [`LatexEnvironment`].
/// 
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct CasesEnvironment;

impl LatexEnvironment for CasesEnvironment {
    fn write_name<W: core::fmt::Write>(dest: &mut W) -> Result<(), core::fmt::Error> {
        write!(dest, "cases")
    }
}