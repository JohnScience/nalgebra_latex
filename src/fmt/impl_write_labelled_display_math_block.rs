use nalgebra::{Matrix, Dim, RawStorage};
use super::{WriteLabelledDisplayMathBlock, PlainMatrixFormatter, ParenthesizedMatrixFormatter, BracketedMatrixFormatter, BracedMatrixFormatter, VBarDelimitedMatrixFormatter, DoubleVBarDelimitedMatrixFormatter,
WriteAsLatex};
use crate::latex_modes::DisplayMathMode;

macro_rules! decl_for_matrix_formatter {
    ($formatter:ident) => {
        impl<T,R,C,S> WriteLabelledDisplayMathBlock<Matrix<T,R,C,S>> for $formatter 
        where
            T: WriteAsLatex<DisplayMathMode>,
            R: Dim,
            C: Dim,
            S: RawStorage<T,R,C>,
        {}
    };
}

decl_for_matrix_formatter!(PlainMatrixFormatter);
decl_for_matrix_formatter!(ParenthesizedMatrixFormatter);
decl_for_matrix_formatter!(BracketedMatrixFormatter);
decl_for_matrix_formatter!(BracedMatrixFormatter);
decl_for_matrix_formatter!(VBarDelimitedMatrixFormatter);
decl_for_matrix_formatter!(DoubleVBarDelimitedMatrixFormatter);
