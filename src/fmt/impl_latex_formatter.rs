use nalgebra::{Dim, Matrix, RawStorage};

use super::{
    BracedMatrixFormatter, BracketedMatrixFormatter, DoubleVBarDelimitedMatrixFormatter,
    LatexFormatter, ParenthesizedMatrixFormatter, PlainMatrixContentsFormatter,
    PlainMatrixFormatter, VBarDelimitedMatrixFormatter, WriteAsLatex,
};
use crate::{
    env::{
        BracedMatrixEnvironment, BracketedMatrixEnvironment, DoubleVBarDelimitedMatrixEnvironment,
        LatexEnvironment, ParenthesizedMatrixEnvironment, PlainMatrixEnvironment,
        VBarDelimitedMatrixEnvironment,
    },
    latex_modes::{
        CategorizedLatexModeKind, CategorizedLatexModeKindExt, CategoryEnumVariantExt,
        ControlSeqDelimited, MathLatexMode, MathLatexModeKind,
    },
};

use core::fmt::{Error, Write};

impl<M, T, R, C, S> LatexFormatter<M, M, Matrix<T, R, C, S>> for PlainMatrixContentsFormatter
where
    M: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
    T: WriteAsLatex<M>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    // Note: the implementation is nearly identical to the implementation of UncheckedLatexFormatter
    fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T, R, C, S>) -> Result<(), Error> {
        let nrows = m.nrows();
        let ncols = m.ncols();

        for i in 0..nrows {
            for j in 0..ncols {
                <T as WriteAsLatex<M>>::write_as_latex(&m[(i, j)], dest)?;
                if j != ncols - 1 {
                    dest.write_str("&")?;
                }
            }
            if i != nrows - 1 {
                dest.write_str(r"\\")?;
            }
        }
        Ok(())
    }
}

macro_rules! decl_matrix_formatter {
    ($formatter:ident for $environment:ident) => {
        impl<IM, OM, T, R, C, S> LatexFormatter<IM, OM, Matrix<T, R, C, S>> for $formatter
        where
            IM: CategorizedLatexModeKindExt,
            OM: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind> + ControlSeqDelimited,
            T: WriteAsLatex<OM>,
            R: Dim,
            C: Dim,
            S: RawStorage<T, R, C>,
        {
            fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T, R, C, S>) -> Result<(), Error> {
                use CategorizedLatexModeKind::*;
                let is_delimiting_required = match IM::CATEGORIZED_KIND {
                    eq if eq == Math(OM::CATEGORY_ENUM_VARIANT) => Ok(false),
                    Math(_) => Err(Error),
                    _ => Ok(true),
                }?;
                if is_delimiting_required {
                    OM::write_opening_control_seq(dest)?;
                };
                <$environment>::write_opening_tag(dest)?;
                PlainMatrixContentsFormatter::write_latex(dest, m)?;
                <$environment>::write_closing_tag(dest)?;
                if is_delimiting_required {
                    OM::write_closing_control_seq(dest)?;
                };
                Ok(())
            }
        }
    };
}

decl_matrix_formatter!(PlainMatrixFormatter for PlainMatrixEnvironment);
decl_matrix_formatter!(ParenthesizedMatrixFormatter for ParenthesizedMatrixEnvironment);
decl_matrix_formatter!(BracketedMatrixFormatter for BracketedMatrixEnvironment);
decl_matrix_formatter!(BracedMatrixFormatter for BracedMatrixEnvironment);
decl_matrix_formatter!(VBarDelimitedMatrixFormatter for VBarDelimitedMatrixEnvironment);
decl_matrix_formatter!(DoubleVBarDelimitedMatrixFormatter for DoubleVBarDelimitedMatrixEnvironment);
