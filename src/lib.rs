#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test, doctest, feature = "lin_sys")), no_std)]
#![cfg_attr(feature = "adt_const_params", feature(adt_const_params))]

pub mod env;
pub mod fmt;
#[cfg(feature = "lin_sys")]
pub mod lin_sys;

#[cfg(test)]
mod tests {
    use crate::fmt::{LatexFormatter, PlainMatrixContentsFormatter, PlainMatrixFormatter};
    use nalgebra::matrix;

    #[test]
    fn plain_matrix_contents_formatter_works() {
        let mut s = String::new();
        let m = matrix!(
            1,2,3,4;
            5,6,7,8;
            9,10,11,12;
        );
        PlainMatrixContentsFormatter::write_latex(&mut s, &m).unwrap();
        assert_eq!(s, r"1&2&3&4\\5&6&7&8\\9&10&11&12");
    }

    #[test]
    fn plain_matrix_formatter_works() {
        let mut s = String::new();
        let m = matrix!(
            1,2,3,4;
            5,6,7,8;
            9,10,11,12;
        );
        PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
        assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
    }
}
