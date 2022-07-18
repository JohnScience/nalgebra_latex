//! A module offering a number of [LaTeX] formatters for entities from [`nalgebra_linsys`].
//!
//! [`nalgebra_linsys`]: https://crates.io/crates/nalgebra_linsys
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

use core::fmt::Write;

use crate::{
    env::LatexEnvironment,
    fmt::{write_latex, LatexFormatter, WriteAsLatex},
    latex_modes::{
        CategorizedLatexModeKind, CategorizedLatexModeKindExt, CategoryEnumVariantExt,
        ControlSeqDelimited, MathLatexMode, MathLatexModeKind,
    },
    lin_sys::{env::CasesEnvironment, unknowns::Unknowns, LinSys},
};
use nalgebra::{Dim, RawStorage};

/// Plain ["environment"]-agnostic [LaTeX] formatter for [linear systems], e.g.
/// `1x_{0}+0x_{1}=3x\\4x_{0}+5x_{1}=6`.
///
/// # Example
///
/// ```
/// use nalgebra::{matrix, Dynamic};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
///         fmt::PlainLinSysFormatter,
///     },
///     fmt::{write_latex, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3;
///     4,5,6;
///     7,8,9;
/// );
/// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new(
///     'x',
///     // use nalgebra::Const if you can keep the size unchanged
///     Dynamic::new(2)
/// );
/// let ls = LinSys::new(m, vec_of_unknowns).unwrap();
/// write_latex::<PlainLinSysFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &ls).unwrap();
/// assert_eq!(s, r"1x_{1}+2x_{2}=3\\4x_{1}+5x_{2}=6\\7x_{1}+8x_{2}=9");
/// ```
///
/// # Notes
///
/// This type can be the foundational block of other formatters for [linear systems].
///
/// Zeroes are printed too.
///
/// [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// ["environment"]: https://www.overleaf.com/learn/latex/Environments
pub struct PlainLinSysFormatter;

/// Formatter for [linear systems] using [`CasesEnvironment`], which represents `cases` [environment].
///
/// [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct CasesLinSysFormatter;

impl<IM, OM, T, R, C, S, U> LatexFormatter<IM, OM, LinSys<T, R, C, S, U>> for PlainLinSysFormatter
where
    IM: CategorizedLatexModeKindExt,
    OM: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
    T: WriteAsLatex<OM>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    U: Unknowns,
{
    fn write_latex<W: Write>(
        dest: &mut W,
        input: &LinSys<T, R, C, S, U>,
    ) -> Result<(), core::fmt::Error> {
        let nrows = input.matrix.nrows();
        let ncols = input.matrix.ncols();
        let ncols_sub2 = match ncols.checked_sub(2) {
            Some(n) => n,
            None => return Ok(()),
        };
        for i in 0..nrows {
            for j in 0..ncols_sub2 {
                input.matrix[(i, j)].write_as_latex(dest)?;
                unsafe {
                    input
                        .unknowns
                        .write_latex_for_ith_unchecked::<OM, _>(dest, j)
                }?;
                write!(dest, "+")?;
            }
            input.matrix[(i, ncols_sub2)].write_as_latex(dest)?;
            unsafe {
                input
                    .unknowns
                    .write_latex_for_ith_unchecked::<OM, _>(dest, ncols_sub2)
            }?;
            write!(dest, "=")?;
            input.matrix[(i, ncols_sub2 + 1)].write_as_latex(dest)?;
            if i != nrows - 1 {
                write!(dest, r"\\")?;
            }
        }
        Ok(())
    }
}

impl<IM, OM, T, R, C, S, U> LatexFormatter<IM, OM, LinSys<T, R, C, S, U>> for CasesLinSysFormatter
where
    IM: CategorizedLatexModeKindExt,
    OM: MathLatexMode
        + CategoryEnumVariantExt<MathLatexModeKind>
        + ControlSeqDelimited
        + CategorizedLatexModeKindExt,
    T: WriteAsLatex<OM>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    U: Unknowns,
{
    fn write_latex<W: core::fmt::Write>(
        dest: &mut W,
        input: &LinSys<T, R, C, S, U>,
    ) -> Result<(), core::fmt::Error> {
        use CategorizedLatexModeKind::*;
        let is_delimiting_required = match IM::CATEGORIZED_KIND {
            eq if eq == Math(OM::CATEGORY_ENUM_VARIANT) => Ok(false),
            Math(_) => Err(core::fmt::Error),
            _ => Ok(true),
        }?;
        if is_delimiting_required {
            OM::write_opening_control_seq(dest)?;
        }
        CasesEnvironment::write_opening_tag(dest)?;
        write_latex::<PlainLinSysFormatter, OM, OM, _, _>(dest, input)?;
        CasesEnvironment::write_closing_tag(dest)?;
        if is_delimiting_required {
            OM::write_closing_control_seq(dest)?;
        }
        Ok(())
    }
}
