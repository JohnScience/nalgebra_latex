#![cfg_attr(not(any(doc,test,doctest)), no_std)]

use core::fmt::{Write, Display, Error};

use nalgebra::{Matrix, Dim, RawStorage};

pub trait LatexFormatter<I> {
    fn write_latex<W: Write>(input: &I, dest: &mut W) -> Result<(), Error>;
}

pub struct PlainMatrixContentsFormatter;

pub struct PlainMatrixFormatter;

impl<T,R,C,S> LatexFormatter<Matrix<T,R,C,S>> for PlainMatrixContentsFormatter
where
    T: Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn write_latex<W: Write>(m: &Matrix<T,R,C,S>, dest: &mut W) -> Result<(), Error> {
        let nrows = m.nrows();
        let ncols = m.ncols();

        for i in 0..nrows {
            for j in 0..ncols {
                dest.write_fmt(format_args!("{}", m[(i,j)]))?;
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

impl<T,R,C,S> LatexFormatter<Matrix<T,R,C,S>> for PlainMatrixFormatter
where
    T: Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn write_latex<W: Write>(m: &Matrix<T,R,C,S>, dest: &mut W) -> Result<(), Error> {
        dest.write_str(r"\begin{matrix}")?;
        PlainMatrixContentsFormatter::write_latex(m, dest)?;
        dest.write_str(r"\end{matrix}")
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::matrix;
    use crate::{PlainMatrixFormatter, PlainMatrixContentsFormatter, LatexFormatter};

    #[test]
    fn plain_matrix_contents_formatter_works() {
        let mut s = String::new();
        let m = matrix!(
            1,2,3,4;
            5,6,7,8;
            9,10,11,12;
        );
        PlainMatrixContentsFormatter::write_latex(&m, &mut s).unwrap();
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
        PlainMatrixFormatter::write_latex(&m, &mut s).unwrap();
        assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
    }
}
