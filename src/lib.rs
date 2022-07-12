#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test, doctest, feature = "lin_sys")), no_std)]
#![cfg_attr(feature = "adt_const_params", feature(adt_const_params))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod env;
pub mod fmt;
pub mod latex_modes;
#[cfg(feature = "lin_sys")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "lin_sys")))]
pub mod lin_sys;
