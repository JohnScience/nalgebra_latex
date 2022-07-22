#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test, doctest, feature = "lin_sys")), no_std)]
#![cfg_attr(feature = "adt_const_params", feature(adt_const_params))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod env;
pub mod fmt;
pub mod latex_features;
pub mod latex_flavors;
pub mod latex_modes;
pub mod latex_writer;
pub mod lin_sys;

mod macros {
    #[macro_export]
    macro_rules! latex_format {
        ($w:ident += ) => {};
        ($w:ident += $s:literal; $($tail:tt)*) => {
            $w.write_str($s);
        };
    }
}
