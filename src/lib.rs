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

mod macros {
    /// A macro for relatively convenient writing of LaTeX code.
    /// 
    /// # Example
    /// 
    /// ```
    /// use core::fmt::Write;
    /// use nalgebra_latex::inplace_format;
    /// use partial_application::partial;
    /// 
    /// let mut s = String::new();
    /// inplace_format!(s +=
    ///     "It works with string literals.\n" ;
    ///     let (q, a) = ("Does it work with weird let statements?\n", "It does!\n");
    ///     |s: &mut String| s.write_str(q).unwrap() ;
    ///     |s: &mut String| s.write_str(a).unwrap() ;
    ///     partial!(Write::write_str => _, "It also supports functional style.")
    ///         => |res: Result::<_,_>| res.unwrap() ;
    /// );
    /// let mut lines = s.lines();
    /// 
    /// assert_eq!(lines.next(), Some("It works with string literals."));
    /// assert_eq!(lines.next(), Some("Does it work with weird let statements?"));
    /// assert_eq!(lines.next(), Some("It does!"));
    /// assert_eq!(lines.next(), Some("It also supports functional style."));
    /// assert_eq!(lines.next(), None);
    /// ```
    #[macro_export]
    macro_rules! inplace_format {
        ($buf:ident += $str:literal ; $($tail:tt)*) => {
            $buf += $str;
            inplace_format!($buf += $($tail)*);
        };
    
        ($buf:ident += let $p:pat = $expr:expr ; $($tail:tt)*) => {
            let $p = $expr;
            inplace_format!($buf += $($tail)*);
        };
    
        ($buf:ident += $closure_or_macro:expr ; $($tail:tt)*) => {
            ($closure_or_macro)(&mut $buf);
            inplace_format!($buf += $($tail)*);
        };
    
        ($buf:ident += $closure_or_macro:expr $(=> $nxt_closure_or_macro:expr)+ ; $($tail:tt)*) => {
            let _inplace_tmp = ($closure_or_macro)(&mut $buf);
            $(
                let _inplace_tmp = ($nxt_closure_or_macro)(_inplace_tmp);
            )+
            inplace_format!($buf += $($tail)*);
        };
        ($buf:ident += ) => {};
    }
}