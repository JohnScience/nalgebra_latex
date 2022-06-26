/// The alias for the type of the numbering.
///
/// # Notes
///
/// Check the [documentation of the super module][super] for the purpose of the type alias.
#[cfg(any(not(feature = "adt_const_params"), doc_cfg))]
#[cfg_attr(doc_cfg, doc(cfg(any(not(feature = "adt_const_params"), doc_cfg))))]
pub type NumberingTy = usize;

/// The alias for the type of the numbering.
///
/// # Notes
///
/// Check the [documentation of the super module][super] for the purpose of the module.
#[cfg(any(not(feature = "adt_const_params"), doc_cfg))]
#[cfg_attr(doc_cfg, doc(cfg(any(not(feature = "adt_const_params"), doc_cfg))))]
#[allow(non_snake_case, non_upper_case_globals)]
pub mod Numbering {
    pub const ZeroBased: usize = 0usize;
    pub const OneBased: usize = 1usize;
}
