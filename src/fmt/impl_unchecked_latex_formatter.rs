use core::fmt::{Error, Write};

use nalgebra::{Dim, Matrix, RawStorage};

use super::{
    BracedMatrixEnvironment, BracedMatrixFormatter, BracketedMatrixEnvironment,
    BracketedMatrixFormatter, DoubleVBarDelimitedMatrixEnvironment,
    DoubleVBarDelimitedMatrixFormatter, ParenthesizedMatrixEnvironment,
    ParenthesizedMatrixFormatter, PlainMatrixContentsFormatter, PlainMatrixEnvironment,
    PlainMatrixFormatter, UncheckedLatexFormatter, VBarDelimitedMatrixEnvironment,
    VBarDelimitedMatrixFormatter,
};
use crate::env::LatexEnvironment;

impl<T, R, C, S> UncheckedLatexFormatter<Matrix<T, R, C, S>> for PlainMatrixContentsFormatter
where
    T: core::fmt::Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    // Note: the implementation is nearly identical to the implementation of LatexFormatter
    unsafe fn write_latex_unchecked<W: Write>(
        dest: &mut W,
        m: &Matrix<T, R, C, S>,
    ) -> Result<(), Error> {
        let nrows = m.nrows();
        let ncols = m.ncols();

        for i in 0..nrows {
            for j in 0..ncols {
                dest.write_fmt(format_args!("${}$", m[(i, j)]))?;
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
        impl<T, R, C, S> UncheckedLatexFormatter<Matrix<T, R, C, S>> for $formatter
        where
            T: core::fmt::Display,
            R: Dim,
            C: Dim,
            S: RawStorage<T, R, C>,
        {
            unsafe fn write_latex_unchecked<W: Write>(
                dest: &mut W,
                m: &Matrix<T, R, C, S>,
            ) -> Result<(), Error> {
                <$environment>::write_opening_tag(dest)?;
                PlainMatrixContentsFormatter::write_latex_unchecked(dest, m)?;
                <$environment>::write_closing_tag(dest)
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
