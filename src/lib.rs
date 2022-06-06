#![cfg_attr(not(any(doc,test,doctest)), no_std)]

use core::fmt::{Write, Display, Error};

use nalgebra::{Matrix, Dim, RawStorage};

/// Implementers of the trait allow formatting of values of type `&I` in the form of [LaTeX] strings.
/// 
/// # Example
/// 
/// ```
/// use nalgebra::matrix;
///	use nalgebra_latex::{PlainMatrixFormatter, LatexFormatter};
///
///	let mut s = String::new();
///	let m = matrix!(
///		1,2,3,4;
///		5,6,7,8;
///		9,10,11,12;
///	);
/// // The fully-qualified syntax <Type as Trait>::method_name is used to demonstrate the origin
/// // of write_latex() method
///	<PlainMatrixFormatter as LatexFormatter<_>>::write_latex(&mut s, &m).unwrap();
///	assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
/// ```
/// 
/// or simply
/// 
/// ```
/// use nalgebra::matrix;
///	use nalgebra_latex::{PlainMatrixFormatter, LatexFormatter};
///
///	let mut s = String::new();
///	let m = matrix!(
///		1,2,3,4;
///		5,6,7,8;
///		9,10,11,12;
///	);
/// PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
///	assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
/// ```
/// 
/// # Notes
/// 
/// * *At the moment of writing, all supplied type-parameters for the type-argument `I` are parameterized
/// types of generic type [`nalegebra::Matrix`].*
/// 
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [`nalegebra::Matrix`]: https://docs.rs/nalgebra/latest/nalgebra/base/struct.Matrix.html
pub trait LatexFormatter<I> {
    /// Writes the value of type `&I` in the form of [LaTeX] string to the given destination
    /// that implements the [`Write`] trait.
    /// 
    /// # Arguments
    /// 
    /// `W` - type argument of the destination, expected to implement the [`Write`] trait. 
    /// 
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    /// 
    /// `dest` - destination to write the formatted [LaTeX] string to.
    /// 
    /// `input` - value of type `&I` to be formatted as [LaTeX] string.
    /// 
    /// # Returns
    /// 
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the formatted [LaTeX] string was successfully
    /// written to the destination and [`Result::Err`] otherwise.
    /// 
    /// # Example
    /// 
    /// ```
    /// use nalgebra::matrix;
    ///	use nalgebra_latex::{PlainMatrixFormatter, LatexFormatter};
    ///
    ///	let mut s = String::new();
    ///	let m = matrix!(
    ///		1,2,3,4;
    ///		5,6,7,8;
    ///		9,10,11,12;
    ///	);
    /// PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
    ///	assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
    /// ```
    /// 
    /// # Errors
    /// 
    /// If the formatting process fails, the error must be returned as the [`Result::Err`] variant of the result
    /// of the method.
    /// 
    /// # Notes
    /// 
    /// * *Implicitly, panics are not meant to happen.*
    /// 
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    fn write_latex<W: Write>(dest: &mut W, input: &I) -> Result<(), Error>;
}

/// Implementers of the trait represent different [LaTeX] [environments].
/// 
/// # Notes
/// 
/// * *`<const ENV: &'static str>` is not a type parameter of the trait
/// because at the moment of writing it is supported only with #![feature(adt_const_params)]*
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environments]: https://www.overleaf.com/learn/latex/Environments
pub trait LatexEnvironment {
    /// Writes name of the [LaTeX] [environment] to the given destination that implements the [`Write`] trait.
    /// 
    /// # Arguments
    /// 
    /// `W` - type argument of the destination, expected to implement the [`Write`] trait.
    /// `dest` - destination to write the name of the [LaTeX] [environment] to.
    /// 
    /// # Returns
    /// 
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the name of the [LaTeX]
    /// [environment] was successfully written to the destination and [`Result::Err`] otherwise.
    /// 
    /// # Example
    /// 
    // TODO: add example
    /// ```
    /// ```
    /// 
    /// *Note: The name of the environment is supplied via a function instead of an associated constant
    /// because the function can be implemented on static string like "smallpbBvVmatrix" that would
    /// be shared between different matrix environments. Such implementation may or may not have lighter
    /// memory footprint, better cache locality, and performance.*
    /// 
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    /// [environment]: https://www.overleaf.com/learn/latex/Environments
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error>;

    fn write_begin<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str(r"\begin{")?;
        Self::write_name(dest)?;
        dest.write_str("}")
    }
    fn write_end<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str(r"\end{")?;
        Self::write_name(dest)?;
        dest.write_str("}")
    }
}

/// As implementor of [`LatexFormatter`]`<`[`nalgebra::Matrix`]`<_,_,_,_>>` trait, this type offers
/// formatting matrices as default ["environment"]-agnostic [LaTeX] matrix contents.
///
/// # Example
/// 
/// ```
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3,4;
///     5,6,7,8;
///     9,10,11,12;
/// );
/// PlainMatrixContentsFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"1&2&3&4\\5&6&7&8\\9&10&11&12");
/// ```
/// 
/// This type is the foundational block for many others matrix formatting types offered by the crate.
/// 
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// ["environment"]: https://www.overleaf.com/learn/latex/Environments
pub struct PlainMatrixContentsFormatter;

pub struct PlainMatrixFormatter;

impl<T,R,C,S> LatexFormatter<Matrix<T,R,C,S>> for PlainMatrixContentsFormatter
where
    T: Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T,R,C,S>) -> Result<(), Error> {
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
    fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T,R,C,S>) -> Result<(), Error> {
        dest.write_str(r"\begin{matrix}")?;
        PlainMatrixContentsFormatter::write_latex(dest, m)?;
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
