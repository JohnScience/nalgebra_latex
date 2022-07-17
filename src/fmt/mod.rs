//! A module offering a number of contents-aware matrix [LaTeX] formatters for [`nalgebra`]
//! as well as [`write_latex`] function to rule them all.
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

#[cfg(feature = "evcxr")]
mod impl_evcxr_output_formatter;
mod impl_latex_formatter;
mod impl_latex_formatter_quadruple;
#[cfg(feature = "evcxr")]
mod impl_unchecked_evcxr_output_formatter;
mod impl_unchecked_latex_formatter;
mod impl_write_as_latex;

use zst::ZST;

use crate::{
    env::{
        BracedMatrixEnvironment, BracketedMatrixEnvironment, DoubleVBarDelimitedMatrixEnvironment,
        ParenthesizedMatrixEnvironment, PlainMatrixEnvironment, VBarDelimitedMatrixEnvironment,
    },
    latex_modes::LatexMode,
};
use core::fmt::{Error, Write};

pub fn write_latex<F, IM, OM, W, I>(dest: &mut W, input: &I) -> Result<(), core::fmt::Error>
where
    F: LatexFormatter<IM, OM, I>,
    IM: LatexMode,
    OM: LatexMode,
    W: core::fmt::Write,
{
    F::write_latex(dest, input)
}

pub trait WriteAsLatex<M>
where
    M: LatexMode,
{
    fn write_as_latex<W: Write>(&self, dest: &mut W) -> Result<(), core::fmt::Error>;
}

pub trait WriteFormated<I> {
    fn write_formated<W: Write>(dest: &mut W, input: &I) -> Result<(), core::fmt::Error>;
}

// TODO: provide a good example with an implementation via type-wrapper
/// Implementers of the trait allow quick-and-dirty by-reference formatting of
/// values of the type-parameter in the form, which is **assumed** to be a valid a [LaTeX]
/// string.
///
/// # Bad example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::fmt::{ParenthesizedMatrixFormatter, UncheckedLatexFormatter};
/// // The `const_num_bigint` crate does not have a first-class support for LaTeX formatting.
/// use const_num_bigint::{BigUint, biguint};
///
///
/// const A11 : &'static BigUint = biguint!("1");
/// const A12 : &'static BigUint = biguint!("2");
/// const A21 : &'static BigUint = biguint!("3");
/// const A22 : &'static BigUint = biguint!("4");
///
/// let M = matrix![
///    A11, A12;
///    A21, A22;
/// ];
///
/// let mut s = String::new();
/// // In good code, unsafe blocks must *always* be commented.
/// // They should explain why their use is safe
/// // based on the `# Unsafety` block of the documentation.
/// #[allow(deprecated)]
/// unsafe { ParenthesizedMatrixFormatter::write_latex_unchecked(&mut s, &M) }.unwrap();
/// assert_eq!(s, r"\begin{pmatrix}1&2\\3&4\end{pmatrix}");
/// ```
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
pub trait UncheckedLatexFormatter<I> {
    /// Writes the value of type `&I` in the form, which is **assumed** to be a [LaTeX] string
    /// into the given "writer", i.e. the destination that implements the [`Write`] trait.
    ///
    /// # Generic parameters
    ///
    /// `W` - type parameter of the destination, expected to implement the [`Write`] trait.
    ///
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    ///
    /// # Arguments
    ///
    /// `dest` - destination into which the formatted **presumable** [LaTeX] string should be written.
    ///
    /// `input` - value of type `&I` which is **assumed** to be formatted as [LaTeX] string.
    ///
    /// # Returns
    ///
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the formatted **presumable** [LaTeX]
    /// string was successfully written to the destination and [`Result::Err`] if it wasn't.
    ///
    /// # Bad example
    ///
    /// ```
    /// use nalgebra::matrix;
    /// use nalgebra_latex::fmt::{ParenthesizedMatrixFormatter, UncheckedLatexFormatter};
    /// // The `const_num_bigint` crate does not have a first-class support for LaTeX formatting.
    /// use const_num_bigint::{BigUint, biguint};
    ///
    ///
    /// const A11 : &'static BigUint = biguint!("1");
    /// const A12 : &'static BigUint = biguint!("2");
    /// const A21 : &'static BigUint = biguint!("3");
    /// const A22 : &'static BigUint = biguint!("4");
    ///
    /// let M = matrix![
    ///    A11, A12;
    ///    A21, A22;
    /// ];
    ///
    /// let mut s = String::new();
    /// // In good code, unsafe blocks must *always* be commented.
    /// // They should explain why their use is safe
    /// // based on the `# Unsafety` block of the documentation.
    /// #[allow(deprecated)]
    /// unsafe { ParenthesizedMatrixFormatter::write_latex_unchecked(&mut s, &M) }.unwrap();
    /// assert_eq!(s, r"\begin{pmatrix}1&2\\3&4\end{pmatrix}");
    /// ```
    ///
    /// # Unsafety
    ///
    /// This is unsafe because the caller must guarantee that the input will be formated as a valid [LaTeX] string.
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    #[deprecated(
        since = "0.1.0",
        note = "In case if you can't implement (a foreign for your crate) trait LatexFormatter on a
foreign type `T` from another upstream crate, you can make a wrapper around `T` and
implement LatexFormatter on the type-wrapper instead. For convenience, you can
use [`delegate`](https://crates.io/crates/delegate) crate."
    )]
    unsafe fn write_latex_unchecked<W: Write>(dest: &mut W, input: &I) -> Result<(), Error>;
}

/// This type acts as a list of arbitrary types of length 4.
///
/// Construction of a tuple with types of unknown size is impossible.
pub(crate) type ZSTQuadruple<Formatter, Input, InitialMode, OutputMode> = (
    ZST<Formatter>,
    ZST<Input>,
    ZST<InitialMode>,
    ZST<OutputMode>,
);

/// Check the impl module for more information.
pub(crate) trait LatexFormatterQuadruple: Sized {
    type Formatter;
    type Input;
    type InitialMode: LatexMode;
    type OutputMode: LatexMode;

    fn write_latex<W: Write>(dest: &mut W, input: &Self::Input) -> Result<(), Error>;
}

/// Implementers of the trait allow by-reference formatting of values in the
/// specific to the given [initial-output LaTeX mode pair][crate::latex_modes]
/// form of [LaTeX] strings.
///
/// # Generic parameters
///
/// `I` - type parameter of the inputs. `input: &I` is one of the parameters when formatting.
///
/// `InitialMode` - type parameter representing [latex mode][crate::latex_modes] from which writing
/// of the [LaTeX] string starts.
///
/// `OutputMode` - type parameter representing [latex mode][crate::latex_modes] in which writing
/// of the [LaTeX] string should happen.
///
/// *Note: In many (if not all) cases, after the write the [LaTeX] machinery would proceed
/// in the initial [latex mode][crate::latex_modes]*.
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::{
///     fmt::{write_latex, PlainMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3,4;
///     5,6,7,8;
///     9,10,11,12;
/// );
///
/// write_latex::<PlainMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}$");
/// s.clear();
/// write_latex::<PlainMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}$$");
/// ```
///
/// # Notes
///
/// * *At the moment of writing, all supplied type-parameters for the type-parameter `I` are parameterized
/// types of generic type [`nalegebra::Matrix`].*
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [`nalegebra::Matrix`]: https://docs.rs/nalgebra/latest/nalgebra/base/struct.Matrix.html
pub trait LatexFormatter<InitialMode, OutputMode, I>
where
    InitialMode: LatexMode,
    OutputMode: LatexMode,
{
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
    /// use nalgebra_latex::{
    ///     fmt::{write_latex, PlainMatrixFormatter, LatexFormatter},
    ///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
    /// };
    ///
    /// let mut s = String::new();
    /// let m = matrix!(
    ///     1,2,3,4;
    ///     5,6,7,8;
    ///     9,10,11,12;
    /// );
    ///
    /// write_latex::<PlainMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
    /// assert_eq!(s, r"$\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}$");
    /// s.clear();
    /// write_latex::<PlainMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
    /// assert_eq!(s, r"$$\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}$$");
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
#[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
/// :dep mime_typed = { version = "0.1.6", features = ["evcxr_support"] }
/// 
/// use nalgebra::{matrix, Const};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
///         fmt::CasesLinSysFormatter,
///     },
///     fmt::UncheckedEvcxrOutputFormatter,
/// };
/// use mime_typed::evcxr_support::TextMarkdownUtf8;
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
/// unsafe { CasesLinSysFormatter::unchecked_write_evcxr_output(&mut s, &ls) }.unwrap();
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
#[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
///
/// use nalgebra::{matrix, Const};
/// use nalgebra_latex::{
///     lin_sys::{
///         LinSys,
///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
///         numbering::Numbering,
///         fmt::CasesLinSysFormatter,
///     },
///     fmt::UncheckedEvcxrOutputFormatter,
/// };
/// use mime_typed::evcxr_support::TextMarkdownUtf8;
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
///
/// unsafe { CasesLinSysFormatter::unchecked_write_evcxr_output(&mut s, &ls) }.unwrap();
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
pub trait UncheckedEvcxrOutputFormatter<I> {
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
    #[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
    /// :dep mime_typed = "0.1.6"
    /// 
    /// use nalgebra::{matrix, Const};
    /// use nalgebra_latex::{
    ///     lin_sys::{
    ///         LinSys,
    ///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
    ///         numbering::Numbering,
    ///         fmt::CasesLinSysFormatter,
    ///     },
    ///     fmt::UncheckedEvcxrOutputFormatter,
    /// };
    /// use mime_typed::evcxr_support::TextMarkdownUtf8;
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
    /// unsafe { CasesLinSysFormatter::unchecked_write_evcxr_output(&mut s, &ls) }.unwrap();
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
    #[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
    ///
    /// use nalgebra::{matrix, Const};
    /// use nalgebra_latex::{
    ///     lin_sys::{
    ///         LinSys,
    ///         unknowns::SingleLetterBoldfaceVecOfUnknowns,
    ///         numbering::Numbering,
    ///         fmt::CasesLinSysFormatter,
    ///     },
    ///     fmt::UncheckedEvcxrOutputFormatter,
    /// };
    /// use mime_typed::evcxr_support::TextMarkdownUtf8;
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
    /// unsafe { CasesLinSysFormatter::unchecked_write_evcxr_output(&mut s, &ls) }.unwrap();
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
    unsafe fn write_evcxr_output_unchecked<M, W>(dest: &mut W, input: &I) -> Result<(), Error>
    where
        M: mime_typed::MimeStrExt,
        W: Write;
}

/// Implementers of the trait allow by-reference formatting of values of type-parameter in the form of
/// [`evcxr`]-supported output with the given [MIME type].
/// 
/// # Generic arguments
/// 
/// `M` - type parameter representing [MIME type]; expected to implement the [`MimeStrExt`] trait.
/// 
/// `I` - type parameter of the inputs; `input: &I` is one of the parameters when formatting.
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
#[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
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
/// 
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
#[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
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
/// [`evcxr`]: https://github.com/google/evcxr
/// [MIME type]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
/// [`evcxr` kernel]: https://github.com/google/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb
/// [Jupyter Notebook]: https://en.wikipedia.org/wiki/Project_Jupyter#Jupyter_Notebook
#[cfg(feature = "evcxr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "evcxr")))]
pub trait EvcxrOutputFormatter<M, I> {
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
    #[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
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
    /// 
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
    #[doc = concat!(":dep nalgebra_latex = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"lin_sys\", \"evcxr\"] }")]
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
    fn write_evcxr_output<W>(&self, dest: &mut W, input: &I) -> Result<(), Error>
    where
        M: mime_typed::MimeStrExt,
        W: Write;
}

/// Plain ["environment"]-agnostic [LaTeX] formatter for matrices' contents, e.g. `1&2&3&4\\5&6&7&8`.
///
/// # Example
///
/// ```
/// use nalgebra::matrix;
/// use nalgebra_latex::{
///     fmt::{write_latex, PlainMatrixContentsFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///     1,2,3,4;
///     5,6,7,8;
///     9,10,11,12;
/// );
///
/// write_latex::<PlainMatrixContentsFormatter,InlineMathMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"1&2&3&4\\5&6&7&8\\9&10&11&12");
/// s.clear();
/// write_latex::<PlainMatrixContentsFormatter,DisplayMathMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
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
/// use nalgebra_latex::{
///     fmt::{write_latex, PlainMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
///
/// write_latex::<PlainMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}$");
/// s.clear();
/// write_latex::<PlainMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{matrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{matrix}$$");
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
/// use nalgebra_latex::{
///     fmt::{write_latex, ParenthesizedMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
///
/// write_latex::<ParenthesizedMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{pmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{pmatrix}$");
/// s.clear();
/// write_latex::<ParenthesizedMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{pmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{pmatrix}$$");
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
/// use nalgebra_latex::{
///     fmt::{write_latex, BracketedMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
///
/// write_latex::<BracketedMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{bmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{bmatrix}$");
/// s.clear();
/// write_latex::<BracketedMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{bmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{bmatrix}$$");
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
/// use nalgebra_latex::{
///     fmt::{write_latex, BracedMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
///
/// write_latex::<BracedMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{Bmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{Bmatrix}$");
/// s.clear();
/// write_latex::<BracedMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{Bmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{Bmatrix}$$");
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
/// use nalgebra_latex::{
///     fmt::{write_latex, VBarDelimitedMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
///
/// write_latex::<VBarDelimitedMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{vmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{vmatrix}$");
/// s.clear();
/// write_latex::<VBarDelimitedMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{vmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{vmatrix}$$");
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
/// use nalgebra_latex::{
///     fmt::{write_latex, DoubleVBarDelimitedMatrixFormatter, LatexFormatter},
///     latex_modes::{InlineMathMode, DisplayMathMode, InnerParagraphMode},
/// };
///
/// let mut s = String::new();
/// let m = matrix!(
///    1,2,3,4;
///    5,6,7,8;
///    9,10,11,12;
/// );
///
/// write_latex::<DoubleVBarDelimitedMatrixFormatter,InnerParagraphMode,InlineMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{Vmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{Vmatrix}$");
/// s.clear();
/// write_latex::<DoubleVBarDelimitedMatrixFormatter,InnerParagraphMode,DisplayMathMode,_,_>(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{Vmatrix}1&2&3&4\\5&6&7&8\\9&10&11&12\end{Vmatrix}$$");
/// ```
///
/// [environment]: https://www.overleaf.com/learn/latex/Environments
pub struct DoubleVBarDelimitedMatrixFormatter;
