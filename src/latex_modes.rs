//! Module with types representing [LaTeX] modes and with traits of those types.
//! 
//! A bit outdated in terms of findings information about the modes is available
//! in
//! [this comment on the "Latex modes are difficult" issue][whining].
//! 
//! Ever since then, the author also found "The TeXbook" by Donald Knuth, which
//! explains the modes in detail. However, Donald Knuth's book was written before
//! LaTeX became a thing and it mentions only 6 modes and does not mention the
//! subcases of the "paragraph" mode.
//! 
//! As opposed to [LaTeX], its subset supported in Markdown has only three modes.
//! The current suggestion is to use
//! 
//! * [InnerParagraphMode] for the "paragraph" mode;
//! * [DisplayMathMode] for the display math mode; and
//! * [InlineMathMode] for the inline math mode.
//! 
//! [whining]: https://github.com/JohnScience/nalgebra_latex/issues/2#issuecomment-1181474307
//! [LaTeX]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F

// http://latexref.xyz/Modes.html
// http://www.personal.ceu.hu/tex/modes.htm

macro_rules! decl_latex_modes {
    ($(category:$cat:ident(trait $cat_trait:ident, enum $cat_enum:ident) {
        $(mode:$mode:ident),+
    }),+) => {
        #[derive(PartialEq, Eq)]
        pub enum LatexModeCategory {
            $(
                $cat,
            )+
        }

        #[derive(Clone,Copy,PartialEq, Eq)]
        pub enum LatexModeKind {
            $(
                $($mode,)+
            )+
        }

        #[derive(PartialEq,Eq)]
        pub enum CategorizedLatexModeKind {
            $(
                $cat($cat_enum),
            )+
        }

        pub trait LatexMode {
            fn kind(&self) -> LatexModeKind;
            fn categorized_kind(&self) -> CategorizedLatexModeKind {
                match self.kind() {
                    $($(
                        LatexModeKind::$mode => CategorizedLatexModeKind::$cat($cat_enum::$mode),
                    )+)+
                }
            }
            fn category(&self) -> LatexModeCategory {
                match self.kind() {
                    $(
                        $(LatexModeKind::$mode)|+ => LatexModeCategory::$cat,
                    )+
                }
            }
        }

        pub trait LatexModeKindExt: LatexMode {
            const KIND: LatexModeKind;
        }

        pub trait CategorizedLatexModeKindExt: LatexModeKindExt {
            const CATEGORIZED_KIND: CategorizedLatexModeKind;
        }

        impl LatexMode for LatexModeKind {
            fn kind(&self) -> LatexModeKind {
                *self
            }
        }

        pub trait CategoryEnumVariantExt<CE> {
            const CATEGORY_ENUM_VARIANT: CE;
        }

        pub trait ControlSeqDelimited {
            fn write_opening_control_seq<W: core::fmt::Write>(w: &mut W) -> Result<(), core::fmt::Error>;
            fn write_closing_control_seq<W: core::fmt::Write>(w: &mut W) -> Result<(), core::fmt::Error>;
        }

        $(
            pub trait $cat_trait: LatexMode {
                fn category_enum(&self) -> $cat_enum;
            }

            #[derive(PartialEq,Eq)]
            pub enum $cat_enum {
                $(
                    $mode,
                )+
            }

            impl LatexMode for $cat_enum {
                fn kind(&self) -> LatexModeKind {
                    match *self {
                        $(
                            $cat_enum::$mode => LatexModeKind::$mode,
                        )+
                    }
                }
            }

            $(
                pub struct $mode;

                impl LatexMode for $mode {
                    fn kind(&self) -> LatexModeKind {
                        LatexModeKind::$mode
                    }
                }

                impl LatexModeKindExt for $mode {
                    const KIND: LatexModeKind = LatexModeKind::$mode;
                }

                impl CategorizedLatexModeKindExt for $mode {
                    const CATEGORIZED_KIND: CategorizedLatexModeKind = CategorizedLatexModeKind::$cat($cat_enum::$mode);
                }

                impl CategoryEnumVariantExt<$cat_enum> for $mode {
                    const CATEGORY_ENUM_VARIANT: $cat_enum = $cat_enum::$mode;
                }

                impl $cat_trait for $mode {
                    fn category_enum(&self) -> $cat_enum {
                        $cat_enum::$mode
                    }
                }
            )+
        )+

        impl ControlSeqDelimited for InlineMathMode {
            fn write_opening_control_seq<W: core::fmt::Write>(w: &mut W) -> Result<(), core::fmt::Error> {
                w.write_char('$')
            }
            fn write_closing_control_seq<W: core::fmt::Write>(w: &mut W) -> Result<(), core::fmt::Error> {
                w.write_char('$')
            }
        }

        impl ControlSeqDelimited for DisplayMathMode {
            fn write_opening_control_seq<W: core::fmt::Write>(w: &mut W) -> Result<(), core::fmt::Error> {
                w.write_str("$$")
            }
            fn write_closing_control_seq<W: core::fmt::Write>(w: &mut W) -> Result<(), core::fmt::Error> {
                w.write_str("$$")
            }
        }
    };
}

decl_latex_modes! {
    category:Horizontal(trait HorizontalLatexMode, enum HorizontalLatexModeKind) {
        mode:OuterParagraphMode,
        mode:InnerParagraphMode,
        mode:LRMode
    },
    category:Math(trait MathLatexMode, enum MathLatexModeKind) {
        mode:InlineMathMode,
        mode:DisplayMathMode
    },
    category:Vertical(trait VerticalLatexMode, enum VerticalLatexModeKind) {
        mode:VerticalMode,
        mode:InternalVerticalMode
    }
}
