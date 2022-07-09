//! A module offering a number of contents-aware matrix [LaTeX] formatters for [`nalgebra`].
//!
//! At the moment of writing, it supports the environments from one of the most popular [LaTeX] packages,
//! [`amsmath`]:
//!
//! * `matrix` - plain matrix without any delimiters, represented by [`PlainMatrixFormatter`];
//! * `pmatrix` - parenthesized matrix, represented by [`ParenthesizedMatrixFormatter`];
//! * `bmatrix` - matrix delimited with brackets, represented by [`BracketedMatrixFormatter`];
//! * `Bmatrix` - matrix delimited with braces, represented by [`BracedMatrixFormatter`];
//! * `vmatrix` - matrix delimited with single vertical bars, represented by [`VBarDelimitedMatrixFormatter`];
//! * `Vmatrix` - matrix delimited with double vertical bars, represented by [`DoubleVBarDelimitedMatrixFormatter`].
//!
//! [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
use crate::env::{
    BracedMatrixEnvironment, BracketedMatrixEnvironment, DoubleVBarDelimitedMatrixEnvironment,
    LatexEnvironment, ParenthesizedMatrixEnvironment, PlainMatrixEnvironment,
    VBarDelimitedMatrixEnvironment,
};
use core::fmt::{Display, Error, Write};
use nalgebra::{Dim, Matrix, RawStorage};

/// Implementers of the trait allow by-reference formatting of values of type-parameter in the form of [LaTeX] strings.
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{PlainMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3,4;
///     5,6,7,8;
///     9,10,11,12;
/// );
/// // The fully-qualified syntax <Type as Trait>::method_name is used to demonstrate the origin
/// // of write_latex() method
/// <PlainMatrixFormatter as LatexFormatter<_>>::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
/// ```
///
/// or simply
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{PlainMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3,4;
///     5,6,7,8;
/// 9,10,11,12;
/// );
/// PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
/// ```
///
/// # Notes
///
/// * *At the moment of writing, all supplied type-parameters for the type-parameter `I` are parameterized
/// types of generic type [`nalegebra::Matrix`].*
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [`nalegebra::Matrix`]: https://docs.rs/nalgebra/latest/nalgebra/base/struct.Matrix.html
pub trait LatexFormatter<I> {
    /// Writes the value of type `&I` in the form of [LaTeX] string into the given "writer", i.e.
    /// the destination that implements the [`Write`] trait.
    ///
    /// # Generic parameters
    ///
    /// `W` - type parameter of the destination, expected to implement the [`Write`] trait.
    ///
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    ///
    /// # Arguments
    ///
    /// `dest` - destination into which the formatted [LaTeX] string should be written.
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
    /// use nalgebra_latex::fmt::{PlainMatrixFormatter, LatexFormatter};
    ///
    /// let mut s = String::new();
    /// let m = matrix!(
    ///     1,2,3,4;
    ///     5,6,7,8;
    ///     9,10,11,12;
    /// );
    /// PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
    /// assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
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

/// Implementers of the trait allow by-reference formatting of values of type-parameter in the form of
/// [`evcxr`]-supported output.
///
/// # Example for [Jupyter Notebook] with [`evcxr` kernel]
///
/// ```ignore
/// :dep execute_evcxr = { version = "0.1.0" }
/// 
/// use execute_evcxr::{execute_evcxr, Config};
/// 
/// let config = Config { verbose: false, ..Default::default() };
/// execute_evcxr(r#"
/// :dep nalgebra = "0.31.0"
/// :dep nalgebra_latex = { version = "0.1.5", features = ["lin_sys", "evcxr"] }
/// 
/// use nalgebra::{matrix, Const};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
///         fmt::CasesLinSysFormatter,
///     },
///     fmt::EvcxrOutputFormatter,
/// };
/// use std::io::{stdout, Write};
/// 
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3;
///     4,5,6;
///     7,8,9;
/// );
/// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
/// let ls = LinSys::new(m, vec_of_unknowns);
/// CasesLinSysFormatter::write_evcxr_output(&mut s, &ls).unwrap();
/// stdout().write_all(s.as_bytes()).unwrap();
/// "#, config);
/// ```
/// # Example for Rust project
/// 
/// ```
/// extern crate execute_evcxr;
/// 
/// use execute_evcxr::{execute_evcxr, Config};
/// 
/// fn main() {
///     let config = Config { ..Config::default() };
///     execute_evcxr(r#"
/// :dep nalgebra = "0.31.0"
/// :dep nalgebra_latex = { version = "0.1.5", features = ["lin_sys", "evcxr"] }
/// 
/// use nalgebra::{matrix, Const};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
///         fmt::CasesLinSysFormatter,
///     },
///     fmt::EvcxrOutputFormatter,
/// };
/// use std::io::{stdout, Write};
/// 
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3;
///     4,5,6;
///     7,8,9;
/// );
/// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
/// let ls = LinSys::new(m, vec_of_unknowns);
/// CasesLinSysFormatter::write_evcxr_output(&mut s, &ls).unwrap();
/// stdout().write_all(s.as_bytes()).unwrap();
/// "#, config);
/// }
/// ```
///
/// # Notes
///
/// * *At the moment of writing, all supplied type-parameters for the type-parameter `I` are parameterized
/// types of generic type [`nalegebra::Matrix`].*
///
/// [`evcxr`]: https://github.com/google/evcxr
/// [`evcxr` kernel]: https://github.com/google/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb
/// [Jupyter Notebook]: https://en.wikipedia.org/wiki/Project_Jupyter#Jupyter_Notebook
/// [`nalegebra::Matrix`]: https://docs.rs/nalgebra/latest/nalgebra/base/struct.Matrix.html
#[cfg(feature = "evcxr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "evcxr")))]
pub trait EvcxrOutputFormatter<I> {
    /// Writes the value of type `&I` in the form of [`evcxr`]-supported output into the given "writer", i.e.
    /// the destination that implements the [`Write`] trait.
    ///
    /// # Generic parameters
    ///
    /// `W` - type parameter of the destination, expected to implement the [`Write`] trait.
    ///
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    ///
    /// # Arguments
    ///
    /// `dest` - destination into which the [`evcxr`]-supported output should be written.
    ///
    /// `input` - value of type `&I` to be formatted as [`evcxr`]-supported output.
    ///
    /// # Returns
    ///
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the [`evcxr`]-supported output was successfully
    /// written to the destination and [`Result::Err`] otherwise.
    ///
    /// # Example for [Jupyter Notebook] with [`evcxr` kernel]
    ///
    /// ```ignore
    /// :dep execute_evcxr = { version = "0.1.0" }
    /// 
    /// use execute_evcxr::{execute_evcxr, Config};
    /// 
    /// let config = Config { verbose: false, ..Default::default() };
    /// execute_evcxr(r#"
    /// :dep nalgebra = "0.31.0"
    /// :dep nalgebra_latex = { version = "0.1.5", features = ["lin_sys", "evcxr"] }
    /// 
    /// use nalgebra::{matrix, Const};
    /// use nalgebra_latex::{
    ///     lin_sys::{
    ///         LinSys,
    ///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
    ///         numbering::Numbering,
    ///         fmt::CasesLinSysFormatter,
    ///     },
    ///     fmt::EvcxrOutputFormatter,
    /// };
    /// use std::io::{stdout, Write};
    /// 
    /// let mut s = String::new();
    /// let m = matrix!(
    ///     1,2,3;
    ///     4,5,6;
    ///     7,8,9;
    /// );
    /// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
    /// let ls = LinSys::new(m, vec_of_unknowns);
    /// CasesLinSysFormatter::write_evcxr_output(&mut s, &ls).unwrap();
    /// stdout().write_all(s.as_bytes()).unwrap();
    /// "#, config);
    /// ```
    /// # Example for Rust project
    /// 
    /// ```
    /// extern crate execute_evcxr;
    /// 
    /// use execute_evcxr::{execute_evcxr, Config};
    /// 
    /// fn main() {
    ///     let config = Config { ..Config::default() };
    ///     execute_evcxr(r#"
    /// :dep nalgebra = "0.31.0"
    /// :dep nalgebra_latex = { version = "0.1.5", features = ["lin_sys", "evcxr"] }
    /// 
    /// use nalgebra::{matrix, Const};
    /// use nalgebra_latex::{
    ///     lin_sys::{
    ///         LinSys,
    ///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
    ///         numbering::Numbering,
    ///         fmt::CasesLinSysFormatter,
    ///     },
    ///     fmt::EvcxrOutputFormatter,
    /// };
    /// use std::io::{stdout, Write};
    /// 
    /// let mut s = String::new();
    /// let m = matrix!(
    ///     1,2,3;
    ///     4,5,6;
    ///     7,8,9;
    /// );
    /// let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<3>);
    /// let ls = LinSys::new(m, vec_of_unknowns);
    /// CasesLinSysFormatter::write_evcxr_output(&mut s, &ls).unwrap();
    /// stdout().write_all(s.as_bytes()).unwrap();
    /// "#, config);
    /// }
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
    /// [`evcxr`]: https://github.com/google/evcxr
    /// [`evcxr` kernel]: https://github.com/google/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb
    /// [Jupyter Notebook]: https://en.wikipedia.org/wiki/Project_Jupyter#Jupyter_Notebook
    fn write_evcxr_output<W: Write>(dest: &mut W, input: &I) -> Result<(), Error>;
}

/// Plain ["environment"]-agnostic [LaTeX] formatter for matrices' contents, e.g. `1&2&3&4\\5&6&7&8`.
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{PlainMatrixContentsFormatter, LatexFormatter};
///
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

/// Formatter for matrices using [`PlainMatrixEnvironment`], which represents `matrix` [environment].
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{PlainMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
/// PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct PlainMatrixFormatter;

/// Formatter for matrices using [`ParenthesizedMatrixEnvironment`], which represents `pmatrix` [environment].
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{ParenthesizedMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
/// ParenthesizedMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{pmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{pmatrix}");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct ParenthesizedMatrixFormatter;

/// Formatter for matrices using [`BracketedMatrixEnvironment`], which represents `bmatrix` [environment].
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{BracketedMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
/// BracketedMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{bmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{bmatrix}");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct BracketedMatrixFormatter;

/// Formatter for matrices using [`BracedMatrixEnvironment`], which represents `Bmatrix` [environment].
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{BracedMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
/// BracedMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{Bmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{Bmatrix}");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct BracedMatrixFormatter;

/// Formatter for matrices using [`VBarDelimitedMatrixEnvironment`], which represents `vmatrix` environment.
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{VBarDelimitedMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
/// VBarDelimitedMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{vmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{vmatrix}");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct VBarDelimitedMatrixFormatter;

/// Formatter for matrices using [`DoubleVBarDelimitedMatrixEnvironment`], which represents `Vmatrix` environment.
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{DoubleVBarDelimitedMatrixFormatter, LatexFormatter};
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
/// DoubleVBarDelimitedMatrixFormatter::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"\begin{Vmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{Vmatrix}");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct DoubleVBarDelimitedMatrixFormatter;

impl<T, R, C, S> LatexFormatter<Matrix<T, R, C, S>> for PlainMatrixContentsFormatter
where
    T: Display,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T, R, C, S>) -> Result<(), Error> {
        let nrows = m.nrows();
        let ncols = m.ncols();

        for i in 0..nrows {
            for j in 0..ncols {
                dest.write_fmt(format_args!("{}", m[(i, j)]))?;
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
        impl<T, R, C, S> LatexFormatter<Matrix<T, R, C, S>> for $formatter
        where
            T: Display,
            R: Dim,
            C: Dim,
            S: RawStorage<T, R, C>,
        {
            fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T, R, C, S>) -> Result<(), Error> {
                <$environment>::write_opening_tag(dest)?;
                PlainMatrixContentsFormatter::write_latex(dest, m)?;
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

#[cfg(feature = "evcxr")]
impl<I,T> EvcxrOutputFormatter<I> for T
where
    T: LatexFormatter<I>
{
    fn write_evcxr_output<W: Write>(dest: &mut W, i: &I) -> Result<(), Error> {
        dest.write_str("EVCXR_BEGIN_CONTENT text/markdown\n")?;
        T::write_latex(dest, i)?;
        dest.write_str("\nEVCXR_END_CONTENT\n")
    }
}
