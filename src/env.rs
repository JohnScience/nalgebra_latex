//! A module offering a number of contents-unaware matrix [LaTeX] [environments].
//! Useful primarily for customization.
//!
//! At the moment of writing, it supports the environments from one of the most popular [LaTeX] packages,
//! [`amsmath`]:
//!
//! * `matrix` - plain matrix without any delimiters, represented by [`PlainMatrixEnvironment`];
//! * `pmatrix` - parenthesized matrix, represented by [`ParenthesizedMatrixEnvironment`];
//! * `bmatrix` - matrix delimited with brackets, represented by [`BracketedMatrixEnvironment`];
//! * `Bmatrix` - matrix delimited with braces, represented by [`BracedMatrixEnvironment`];
//! * `vmatrix` - matrix delimited with single vertical bars, represented by [`VBarDelimitedMatrixEnvironment`];
//! * `Vmatrix` - matrix delimited with double vertical bars, represented by [`DoubleVBarDelimitedMatrixEnvironment`].
//!
//! For matrix formatting, check out [`nalgebra_latex::fmt`][super::fmt] module.
//!
//! # Notes
//!
//! Built-in support for matrix environments from other [LaTeX] packages, such as [`mathtools`],
//! can be considered in the future.
//!
//! You can learn more about [LaTeX] matrix [environments] in the ["Matrices" paragraph on Overleaf].
//!
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
//! [environments]: https://www.overleaf.com/learn/latex/Environments
//! [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
//! [`mathtools`]: https://ctan.org/pkg/mathtools?lang=en
//! ["Matrices" paragraph on Overleaf]: https://www.overleaf.com/learn/latex/Matrices

use core::fmt::{Error, Write};

/// Implementers of the trait represent different [LaTeX] [environments]. The trait itself can be
/// of interest if you want to extend the functionality of the library.
///
/// # Example
///
/// ```
/// use nalgebra::{Matrix, RawStorage, Dim, Matrix2};
/// use nalgebra_latex::{
///     env::LatexEnvironment,
///     fmt::{
///         LatexFormatterQuadruple, 
///         ZSTQuadruple, 
///         LatexFormatter, 
///         PlainMatrixContentsFormatter, 
///         WriteAsLatex,
///     },
///     latex_modes::{
///         InnerParagraphMode,
///         InlineMathMode,
///         DisplayMathMode,
///         CategorizedLatexModeKindExt,
///         CategorizedLatexModeKind,
///         MathLatexMode,
///         CategoryEnumVariantExt,
///         MathLatexModeKind,
///         ControlSeqDelimited,
///     },
/// };
///
/// use core::fmt::{Write, Error, Display};
///
/// struct MyMatrixEnvironment;
///
/// struct MyMatrixFormatter;
///
/// impl LatexEnvironment for MyMatrixEnvironment {
///    fn write_name<W: Write>(dest: &mut W) -> Result<(), core::fmt::Error> {
///       dest.write_str("mymatrix")
///    }
/// }
/// 
/// impl<IM,OM,T,R,C,S> LatexFormatterQuadruple for ZSTQuadruple<MyMatrixFormatter,Matrix<T, R, C, S>,IM,OM>
/// where
///     IM: CategorizedLatexModeKindExt,
///     OM: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind> + ControlSeqDelimited,
///     T: WriteAsLatex<OM>,
///     R: Dim,
///     C: Dim,
///     S: RawStorage<T, R, C>,
/// {
///     fn write_latex<W: Write>(dest: &mut W, m: &Matrix<T,R,C,S>) -> Result<(), Error> {
///         use CategorizedLatexModeKind::*;
///         let is_delimiting_required = match IM::CATEGORIZED_KIND {
///             eq if eq == Math(OM::CATEGORY_ENUM_VARIANT) => Ok(false),
///             Math(_) => Err(Error),
///             _ => Ok(true),
///         }?;
///         if is_delimiting_required {
///             OM::write_opening_control_seq(dest)?;
///         };
///         MyMatrixEnvironment::write_opening_tag(dest)?;
///         PlainMatrixContentsFormatter::write_latex(dest, m)?;
///         MyMatrixEnvironment::write_closing_tag(dest)?;
///         if is_delimiting_required {
///             OM::write_closing_control_seq(dest)?;
///         };
///         Ok(())
///     }
/// }
///
/// let mut s = String::new();
/// let m = Matrix2::new(1, 2, 3, 4);
///
/// <MyMatrixFormatter as LatexFormatter<InnerParagraphMode,InlineMathMode,_>>::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"$\begin{mymatrix}1&2\\3&4\end{mymatrix}$");
/// s.clear();
/// <MyMatrixFormatter as LatexFormatter<InnerParagraphMode,DisplayMathMode,_>>::write_latex(&mut s, &m).unwrap();
/// assert_eq!(s, r"$$\begin{mymatrix}1&2\\3&4\end{mymatrix}$$");
/// ```
///
/// # Notes
///
/// * *`<const ENV: &'static str>` is not a type parameter of the trait
/// because at the moment of writing it is supported only with `#![feature(adt_const_params)]`*
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environments]: https://www.overleaf.com/learn/latex/Environments
pub trait LatexEnvironment {
    /// Writes the name of the [LaTeX] [environment] (e.g. `matrix`) to the given "writer", i.e. destination that
    /// implements the [`Write`] trait.
    ///
    /// # Generic parameters
    ///
    /// `W` - generic type parameter of the destination, expected to implement the [`Write`] trait.
    /// 
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    ///
    /// # Arguments
    ///
    /// `dest` - destination into which the name of the [LaTeX] [environment] should be written
    /// upon success.
    ///
    /// # Returns
    ///
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the name of the [LaTeX]
    /// [environment] was successfully written to the destination and [`Result::Err`] otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use nalgebra_latex::env::{
    ///     LatexEnvironment,
    ///     PlainMatrixEnvironment,
    ///     ParenthesizedMatrixEnvironment,
    ///     BracketedMatrixEnvironment,
    ///     BracedMatrixEnvironment,
    ///     VBarDelimitedMatrixEnvironment,
    ///     DoubleVBarDelimitedMatrixEnvironment,
    /// };
    ///
    /// let mut s = String::new();
    ///
    /// PlainMatrixEnvironment::write_name(&mut s).unwrap();
    /// assert_eq!(s, "matrix");
    /// s.clear();
    ///
    /// ParenthesizedMatrixEnvironment::write_name(&mut s).unwrap();
    /// assert_eq!(s, "pmatrix");
    /// s.clear();
    ///
    /// BracketedMatrixEnvironment::write_name(&mut s).unwrap();
    /// assert_eq!(s, "bmatrix");
    /// s.clear();
    ///
    /// BracedMatrixEnvironment::write_name(&mut s).unwrap();
    /// assert_eq!(s, "Bmatrix");
    /// s.clear();
    ///
    /// VBarDelimitedMatrixEnvironment::write_name(&mut s).unwrap();
    /// assert_eq!(s, "vmatrix");
    /// s.clear();
    ///
    /// DoubleVBarDelimitedMatrixEnvironment::write_name(&mut s).unwrap();
    /// assert_eq!(s, "Vmatrix");
    /// ```
    ///
    /// *Note: The name of the environment is supplied via a function instead of an associated constant
    /// because the function can be implemented on a static string like "smallpbBvVmatrix" that would
    /// be shared between different matrix environments. Such implementation may or may not have lighter
    /// memory footprint, better cache locality, and performance.*
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    /// [environment]: https://www.overleaf.com/learn/latex/Environments
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error>;

    /// Writes the opening tag of the [LaTeX] [environment] (e.g. `\begin{matrix}`) to the given destination
    /// that implements the [`Write`] trait.
    ///
    /// # Generic parameters
    ///
    /// `W` - type parameter of the destination, expected to implement the [`Write`] trait.
    ///
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    /// 
    /// # Arguments
    ///
    /// `dest` - destination into which the opening tag of the [LaTeX] [environment] should be written
    /// upon success.
    ///
    /// # Returns
    ///
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the opening tag of the [LaTeX]
    /// [environment] was successfully written to the destination and [`Result::Err`] otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use nalgebra_latex::env::{LatexEnvironment, PlainMatrixEnvironment};
    ///
    /// let mut s = String::new();
    /// PlainMatrixEnvironment::write_opening_tag(&mut s).unwrap();
    /// assert_eq!(s, r"\begin{matrix}");
    /// ```
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    /// [environment]: https://www.overleaf.com/learn/latex/Environments
    fn write_opening_tag<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str(r"\begin{")?;
        Self::write_name(dest)?;
        dest.write_str("}")
    }

    /// Writes the closing tag of the [LaTeX] [environment] (e.g. `\end{matrix}`) to the given destination
    /// that implements the [`Write`] trait.
    ///
    /// # Generic parameters
    ///
    /// `W` - type parameter of the destination, expected to implement the [`Write`] trait.
    /// 
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    ///
    /// # Arguments
    ///
    /// `dest` - destination into which the closing tag of the [LaTeX] [environment] should be written
    /// upon success.
    ///
    /// # Returns
    ///
    /// [`Result`]`<(), `[`core::fmt::Error`]`>` - [`Result::Ok`] if the closing tag of the [LaTeX]
    /// [environment] was successfully written to the destination and [`Result::Err`] otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use nalgebra_latex::env::{LatexEnvironment, PlainMatrixEnvironment};
    ///
    /// let mut s = String::new();
    /// PlainMatrixEnvironment::write_closing_tag(&mut s).unwrap();
    /// assert_eq!(s, r"\end{matrix}");
    /// ```
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    /// [environment]: https://www.overleaf.com/learn/latex/Environments
    fn write_closing_tag<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str(r"\end{")?;
        Self::write_name(dest)?;
        dest.write_str("}")
    }
}

/// Representation of `matrix` [LaTeX] [environment] provided by popular [`amsmath`] package. Does not
/// have delimiters.
///
/// For examples of usage, see [`LatexEnvironment`].
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
/// [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
pub struct PlainMatrixEnvironment;

/// Representation of `pmatrix` [LaTeX] [environment] provided by popular [`amsmath`] package.
/// Delimited with parentheses.
///
/// For examples of usage, see [`LatexEnvironment`].
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
/// [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
pub struct ParenthesizedMatrixEnvironment;

/// Representation of `bmatrix` [LaTeX] [environment] provided by popular [`amsmath`] package.
/// Delimited with brackets.
///
/// For examples of usage, see [`LatexEnvironment`].
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
/// [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
pub struct BracketedMatrixEnvironment;

/// Representation of `Bmatrix` [LaTeX] [environment] provided by popular [`amsmath`] package.
/// Delimited with curly braces.
///
/// For examples of usage, see [`LatexEnvironment`].
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
/// [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
pub struct BracedMatrixEnvironment;

/// Representation of `vmatrix` [LaTeX] [environment] provided by popular [`amsmath`] package.
/// Delimited with vertical bars.
///
/// For examples of usage, see [`LatexEnvironment`].
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
/// [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
pub struct VBarDelimitedMatrixEnvironment;

/// Representation of `Vmatrix` [LaTeX] [environment] provided by popular [`amsmath`] package.
/// Delimited with double vertical bars.
///
/// For examples of usage, see [`LatexEnvironment`].
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [environment]: https://www.overleaf.com/learn/latex/Environments
/// [`amsmath`]: https://ctan.org/pkg/amsmath?lang=en
pub struct DoubleVBarDelimitedMatrixEnvironment;

impl LatexEnvironment for PlainMatrixEnvironment {
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str("matrix")
    }
}

impl LatexEnvironment for ParenthesizedMatrixEnvironment {
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str("pmatrix")
    }
}

impl LatexEnvironment for BracketedMatrixEnvironment {
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str("bmatrix")
    }
}

impl LatexEnvironment for BracedMatrixEnvironment {
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str("Bmatrix")
    }
}

impl LatexEnvironment for VBarDelimitedMatrixEnvironment {
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str("vmatrix")
    }
}

impl LatexEnvironment for DoubleVBarDelimitedMatrixEnvironment {
    fn write_name<W: Write>(dest: &mut W) -> Result<(), Error> {
        dest.write_str("Vmatrix")
    }
}
