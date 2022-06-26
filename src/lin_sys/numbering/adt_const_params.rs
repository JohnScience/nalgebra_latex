/// The alias for the type of the numbering.
///
/// # Notes
///
/// Check the [documentation of the super module][super] for the purpose of the type alias.
#[cfg(feature = "adt_const_params")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "adt_const_params")))]
pub type NumberingTy = Numbering;

/// A kind of numbering used for [LaTeX] formatting.
///
/// # Notes
///
/// Check the [documentation of the super module][super] for the purpose of the enum.
///
/// [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
#[cfg(feature = "adt_const_params")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "adt_const_params")))]
#[derive(PartialEq, Eq)]
pub enum Numbering {
    ZeroBased,
    OneBased,
}
