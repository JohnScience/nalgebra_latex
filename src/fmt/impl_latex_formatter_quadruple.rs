/// Quadruple trait appeared to combat E207 that arose in
/// https://github.com/JohnScience/nalgebra_latex/blob/18c7b6f2d4a0f0bde9170d947295542422d8570e/src/fmt/impl_evcxr_output_formatter.rs#L5-L16
///
/// Learn more about E207 here: https://doc.rust-lang.org/error-index.html#E0207
use core::fmt::{Error, Write};

use nalgebra::{Dim, Matrix, RawStorage};

use crate::{
    env::{
        BracedMatrixEnvironment, BracketedMatrixEnvironment, DoubleVBarDelimitedMatrixEnvironment,
        LatexEnvironment, ParenthesizedMatrixEnvironment, PlainMatrixEnvironment,
        VBarDelimitedMatrixEnvironment,
    },
    latex_modes::{
        CategorizedLatexModeKindExt, CategoryEnumVariantExt, ControlSeqDelimited, MathLatexMode,
        MathLatexModeKind,
    },
};

use super::{
    BracedMatrixFormatter, BracketedMatrixFormatter, DoubleVBarDelimitedMatrixFormatter,
    LatexFormatterQuadruple, ParenthesizedMatrixFormatter, PlainMatrixContentsFormatter,
    PlainMatrixFormatter, VBarDelimitedMatrixFormatter, WriteAsLatex, ZSTQuadruple,
};

impl<M, T, R, C, S> LatexFormatterQuadruple
    for ZSTQuadruple<PlainMatrixContentsFormatter, M, M, Matrix<T, R, C, S>>
where
    M: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
    T: WriteAsLatex<M>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    type Formatter = PlainMatrixContentsFormatter;
    type Input = Matrix<T, R, C, S>;
    type InitialMode = M;
    type OutputMode = M;

    fn write_latex<W: Write>(dest: &mut W, m: &Self::Input) -> Result<(), Error> {
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

macro_rules! decl_matrix_formatter_quadruple {
    ($formatter:ident for $environment:ident) => {
        impl<IM, OM, T, R, C, S> LatexFormatterQuadruple
            for ZSTQuadruple<$formatter,IM,OM,Matrix<T, R, C, S>>
        where
            IM: CategorizedLatexModeKindExt,
            OM: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind> + ControlSeqDelimited,
            T: WriteAsLatex<OM>,
            R: Dim,
            C: Dim,
            S: RawStorage<T, R, C>,
        {
            type Formatter = $formatter;
            type Input = Matrix<T, R, C, S>;
            type InitialMode = IM;
            type OutputMode = OM;

            fn write_latex<W: Write>(dest: &mut W, m: &Self::Input) -> Result<(), Error> {
                use crate::latex_modes::CategorizedLatexModeKind::*;
                let is_delimiting_required = match IM::CATEGORIZED_KIND {
                    eq if eq == Math(OM::CATEGORY_ENUM_VARIANT) => Ok(false),
                    Math(_) => Err(Error),
                    _ => Ok(true),
                }?;
                if is_delimiting_required {
                    OM::write_opening_control_seq(dest)?;
                };
                <$environment>::write_opening_tag(dest)?;
                ZSTQuadruple::<PlainMatrixContentsFormatter,OM,OM,Matrix<T,R,C,S>>::write_latex(dest, m)?;
                <$environment>::write_closing_tag(dest)?;
                if is_delimiting_required {
                    OM::write_closing_control_seq(dest)?;
                };
                Ok(())
            }
        }
    };
}

decl_matrix_formatter_quadruple!(PlainMatrixFormatter for PlainMatrixEnvironment);
decl_matrix_formatter_quadruple!(ParenthesizedMatrixFormatter for ParenthesizedMatrixEnvironment);
decl_matrix_formatter_quadruple!(BracketedMatrixFormatter for BracketedMatrixEnvironment);
decl_matrix_formatter_quadruple!(BracedMatrixFormatter for BracedMatrixEnvironment);
decl_matrix_formatter_quadruple!(VBarDelimitedMatrixFormatter for VBarDelimitedMatrixEnvironment);
decl_matrix_formatter_quadruple!(DoubleVBarDelimitedMatrixFormatter for DoubleVBarDelimitedMatrixEnvironment);
