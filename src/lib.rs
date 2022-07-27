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
        (@handle_possible_error $res:ident ?) => {
            $res?
        };

        (@handle_possible_error $res:ident unwrap) => {
            $res.unwrap()
        };

        (@handle_possible_error $res:ident $unknown_strategy:tt) => {
            compile_error!("Unknown format error handling strategy.
                Try using #[on_format_error(unwrap)] or #[on_format_error(?)]");
        };

        (#[on_format_error($strategy:tt)] $w:ident += ) => {};
        (#[on_format_error($strategy:tt)] $w:ident += "$$" ; $($tail:tt)*) => {
            let $w = {
                use ::nalgebra_latex::latex_writer::LatexWriter as LW;
                let res: Result<_, ::core::fmt::Error> = <_ as LW>::write_two_dollar_signs($w);
                latex_format!(@handle_possible_error res $strategy)
            };
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        };
        (#[on_format_error($strategy:tt)] $w:ident += $str:literal; $($tail:tt)*) => {
            let mut $w = $w;
            let $w = {
                use ::nalgebra_latex::latex_writer::UnsafeWrite;
                let res: Result<_, ::core::fmt::Error> = unsafe { <_ as UnsafeWrite>::write_str(&mut $w, $str) }
                    .map(move |_| $w);
                latex_format!(@handle_possible_error res $strategy)
            };
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        };
        (#[on_format_error($strategy:tt)] $w:ident +=
            let mut $v:ident $(: $t:ty)? = $expr:expr; $($tail:tt)*
        ) => {
            let mut $v $(: $t)? = $expr;
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        };
        (#[on_format_error($strategy:tt)] $w:ident +=
            let $v:ident $(: $t:ty)? = $expr:expr; $($tail:tt)*
        ) => {
            let $v $(: $t)? = $expr;
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        };
        (#[on_format_error($strategy:tt)] $w:ident +=
            let mut ($($v:ident),+) $(: $t:ty)? = $expr:expr; $($tail:tt)*
        ) => {
            let mut ($($v),+) $(: $t)? = $expr;
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        };
        (#[on_format_error($strategy:tt)] $w:ident +=
            let ($($v:ident),+) $(: $t:ty)? = $expr:expr; $($tail:tt)*
        ) => {
            let ($($v),+) $(: $t)? = $expr;
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        };
        (#[on_format_error($strategy:tt)] $w:ident += $write_as_latex_implementor:expr; $($tail:tt)*) => {
            let $w = {
                use ::nalgebra_latex::fmt::ConsumingWriteAsLatex;
                use ::nalgebra_latex::latex_writer::LatexWriter as LW;

                let res = $write_as_latex_implementor.consuming_write_as_latex($w);
                latex_format!(@handle_possible_error res $strategy)
            };
            latex_format!(#[on_format_error($strategy)] $w += $($tail)*);
        }
    }
}
