#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc,test,doctest)), no_std)]

pub mod env;
pub mod fmt;

#[cfg(test)]
mod tests {
    use nalgebra::matrix;
    use crate::fmt::{PlainMatrixFormatter, PlainMatrixContentsFormatter, LatexFormatter};

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
