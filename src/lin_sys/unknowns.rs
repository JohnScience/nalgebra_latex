//! A module offering types for "unknowns" as entities that can be presented in [LaTeX]
//!
//! # Notes
//!
//! The term "vector of unknowns" is frequently, if not primarily, used with respect to
//! variables over which [linear systems] are defined.
//!
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
//! [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations

use crate::{
    latex_modes::{
        CategorizedLatexModeKindExt, CategoryEnumVariantExt, MathLatexMode, MathLatexModeKind,
    },
    lin_sys::{
        err::OutOfBoundsError,
        numbering::{Numbering, NumberingTy},
    },
};
use core::{fmt::Write, marker::PhantomData};
use either::Either;
use nalgebra::Dim;

/// Its implementors represent so-called "vectors of unknowns"
///
/// # Notes
///
/// The term "vector of unknowns" is frequently, if not primarily, used with respect to
/// variables over which [linear systems] are defined.
///
/// [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations
pub trait Unknowns {
    /// Writes the representation of the "vector of unknowns" as [LaTeX], e.g. `\textbf{x}` for **x**
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
    /// `dest` - destination into which the [LaTeX] representation of the vector of unknowns should
    /// be written upon success.
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    fn write_latex<IM, OM, W>(&self, dest: &mut W) -> Result<(), core::fmt::Error>
    where
        IM: CategorizedLatexModeKindExt,
        OM: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
        W: Write;
    /// Validates whether the [zero-based index] corresponds to some unknown in the vector of unknowns,
    /// i.e. is within bounds
    ///
    /// # Arguments
    ///
    /// `zbi` - the [zero-based index] that is tested for corresponding to some unknown in the vector of unknowns,
    /// i.e. on which the bound checking is performed.
    ///
    /// [zero-based index]: https://en.wikipedia.org/wiki/Zero-based_numbering
    fn validate_idx(&self, zbi: usize) -> Result<(), OutOfBoundsError>;
    /// Writes the representation of the `i`th [from 0] unknown in the "vector of unkowns" as [LaTeX]
    /// without validating the index. The write is performed into the given "writer", i.e. the destination
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
    /// `dest` - destination into which the `i`th unknown from the vector of unknowns should be written as [LaTeX].
    ///
    /// `zbi` - [zero-based index], also referenced here as `i`, that is expected to correspond to some unknown
    /// from the vector of unknowns.
    ///
    /// # Safety
    ///
    /// [`Unknowns::validate_idx`] must complete successfuly for the given [zero-based index]
    ///
    /// # Notes
    ///
    /// Its safe counterpart is [`Unknowns::write_latex_for_ith`].
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    /// [zero-based index]: https://en.wikipedia.org/wiki/Zero-based_numbering
    unsafe fn write_latex_for_ith_unchecked<M, W>(
        &self,
        dest: &mut W,
        zbi: usize,
    ) -> Result<(), core::fmt::Error>
    where
        M: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
        W: Write;

    /// Validates the [zero-based index] `zbi` and upon success writes the representation of the `i`th unknown
    /// in the "vector of unkowns" as [LaTeX]
    ///
    /// # Generic parameters
    ///
    /// `W` - type parameter of the destination, expected to implement the [`Write`] trait.
    ///
    /// *Note: one notable implementor of [`Write`] trait is [`String`].*
    ///
    /// # Arguments
    ///
    /// `dest` - destination into which the `i`th unknown from the vector of unknowns should be written as [LaTeX].
    ///
    /// `zbi` - [zero-based index], also referenced here as `i`, that is expected to correspond to some unknown
    ///
    /// # Notes
    ///
    /// Its `unsafe` counterpart is [`Unknowns::write_latex_for_ith_unchecked`].
    ///
    /// The type of `Err` variant may or may not be changed in the future to an enum. Please
    /// voice your opinion in the [dedicated issue].
    ///
    /// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
    /// [zero-based index]: https://en.wikipedia.org/wiki/Zero-based_numbering
    /// [dedicated issue]: https://github.com/JohnScience/nalgebra_latex/issues/1
    fn write_latex_for_ith<M, W>(
        &self,
        dest: &mut W,
        zbi: usize,
    ) -> Result<(), Either<core::fmt::Error, OutOfBoundsError>>
    where
        M: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
        W: Write,
    {
        self.validate_idx(zbi).map_err(Either::Right)?;
        unsafe {
            self.write_latex_for_ith_unchecked::<M, W>(dest, zbi)
                .map_err(Either::Left)
        }
    }
}

/// Type of "vector of unknowns" where the name of the vector is one letter, e.g. `x`, and the
/// vector is presented in boldface [LaTeX], e.g. `\textbf{x}`, as opposed to in arrow notation, e.g.
/// `\overrightarrow{x}`.
///
/// # Generic parameters
///
/// `L` - the type of dimension (=length) of the vector of unknowns. When the type implements [`Dim`] trait,
/// extra functionality is provided.
///
/// `N` - constant parameter, the kind of numbering used for formatting of indices of entries in the
/// vector of unknowns.
///
/// # Notes
///
/// The term "vector of unknowns" is frequently, if not primarily, used with respect to
/// variables over which [linear systems] are defined.
///
/// Existence of type parameter `L` can allow either runtime or compile-time access to the length of the vector.
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
/// [linear systems]: https://en.wikipedia.org/wiki/System_of_linear_equations
pub struct SingleLetterBoldfaceVecOfUnknowns<L, const N: NumberingTy> {
    /// The name of the vector of unknowns, e.g. **x**
    pub c: char,
    /// The length of the vector of unknowns
    pub len: L,
    // the private field forbids the usage of "default contructor"
    phantom: PhantomData<()>,
}

impl<L, const N: NumberingTy> SingleLetterBoldfaceVecOfUnknowns<L, N> {
    #[cfg_attr(not(feature = "adt_const_params"), allow(non_upper_case_globals))]
    pub fn new(c: char, len: L) -> Self {
        use Numbering::*;
        debug_assert!(matches!(N, ZeroBased | OneBased));
        Self {
            c,
            len,
            phantom: PhantomData::<()>,
        }
    }
}

impl<L, const N: NumberingTy> Unknowns for SingleLetterBoldfaceVecOfUnknowns<L, N>
where
    L: Copy + Dim,
{
    fn write_latex<IM, OM, W>(&self, w: &mut W) -> Result<(), core::fmt::Error>
    where
        IM: CategorizedLatexModeKindExt,
        OM: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
        W: Write,
    {
        w.write_fmt(format_args!("\\textbf{{{}}}", self.c))
    }

    fn validate_idx(&self, zbi: usize) -> Result<(), OutOfBoundsError> {
        if zbi >= self.len.value() {
            Err(OutOfBoundsError)
        } else {
            Ok(())
        }
    }

    #[cfg_attr(not(feature = "adt_const_params"), allow(non_upper_case_globals))]
    unsafe fn write_latex_for_ith_unchecked<M, W>(
        &self,
        w: &mut W,
        zbi: usize,
    ) -> Result<(), core::fmt::Error>
    where
        M: MathLatexMode + CategoryEnumVariantExt<MathLatexModeKind>,
        W: Write,
    {
        use Numbering::*;

        w.write_fmt(format_args!(
            "{}_{{{}}}",
            self.c,
            match N {
                ZeroBased => zbi,
                OneBased => zbi + 1,
                #[cfg_attr(feature = "adt_const_params", allow(unreachable_patterns))]
                _ => panic!("unsupported numbering"),
            }
        ))
    }
}
