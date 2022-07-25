//! A module offering a number of [LaTeX] formatters for entities from [`nalgebra_linsys`].
//!
//! [`nalgebra_linsys`]: https://crates.io/crates/nalgebra_linsys
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

mod impl_format_as_labelled_display_math_block;
mod impl_latex_formatter;

pub struct PlainLinSysFormatter;

pub struct CasesLinSysFormatter;
