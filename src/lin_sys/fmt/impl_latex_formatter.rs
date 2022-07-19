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

use super::{CasesLinSysFormatter, PlainLinSysFormatter};

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
