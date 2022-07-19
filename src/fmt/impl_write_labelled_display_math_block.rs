use super::{
    BracedMatrixFormatter, BracketedMatrixFormatter, DoubleVBarDelimitedMatrixFormatter,
    ParenthesizedMatrixFormatter, PlainMatrixFormatter, VBarDelimitedMatrixFormatter, WriteAsLatex,
    WriteLabelledDisplayMathBlock,
};
use crate::latex_modes::DisplayMathMode;
use nalgebra::{Dim, Matrix, RawStorage};

macro_rules! decl_for_matrix_formatter {
    ($formatter:ident) => {
        impl<T, R, C, S> WriteLabelledDisplayMathBlock<Matrix<T, R, C, S>> for $formatter
        where
            T: WriteAsLatex<DisplayMathMode>,
            R: Dim,
            C: Dim,
            S: RawStorage<T, R, C>,
        {
        }
    };
}

decl_for_matrix_formatter!(PlainMatrixFormatter);
decl_for_matrix_formatter!(ParenthesizedMatrixFormatter);
decl_for_matrix_formatter!(BracketedMatrixFormatter);
decl_for_matrix_formatter!(BracedMatrixFormatter);
decl_for_matrix_formatter!(VBarDelimitedMatrixFormatter);
decl_for_matrix_formatter!(DoubleVBarDelimitedMatrixFormatter);
