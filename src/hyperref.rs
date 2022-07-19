use core::fmt::{Write, Error};

use crate::latex_modes::{LatexModeKindExt, LatexModeKind};

pub struct Hyperlink {
    referent: &'static str,
    reference: &'static str,
}

macro_rules! decl_hyperlink_supported_latex_flavors {
    ($($flavor:ident,)+) => {
        pub enum HyperlinkSupportedLatexFlavorKind {
            $(
                $flavor,
            )+
        }

        pub trait HyperlinkSupportedLatexFlavorKindExt {
            const KIND: HyperlinkSupportedLatexFlavorKind;
        }

        $(

            impl HyperlinkSupportedLatexFlavorKindExt for crate::latex_flavors::$flavor {
                const KIND: HyperlinkSupportedLatexFlavorKind = HyperlinkSupportedLatexFlavorKind::$flavor;
            }
        )+
    };
}

decl_hyperlink_supported_latex_flavors!(
    AmsLatex,
    MathJax,
);

impl Hyperlink {
    pub fn new(referent: &'static str, reference: &'static str) -> Self {
        Self {
            referent,
            reference,
        }
    }

    pub fn refer<L,M,W>(&self, w: &mut W) -> Result<(), Error>
    where
        L: HyperlinkSupportedLatexFlavorKindExt,
        M: LatexModeKindExt,
        W: Write,
    {
        let is_hyperlink_supported = match L::KIND {
            HyperlinkSupportedLatexFlavorKind::AmsLatex => {
                true
            },
            HyperlinkSupportedLatexFlavorKind::MathJax => {
                false
            },
        };

        if is_hyperlink_supported {
            w.write_str(r"\hyperlink{")?;
            w.write_str(self.referent)?;
            w.write_str("}{")?;
        }
        if !is_hyperlink_supported && M::KIND == LatexModeKind::InnerParagraphMode {
            w.write_str(r"*")
        } else {
            w.write_str(r"\textit{")
        }?;
        w.write_str(self.reference)?;
        if !is_hyperlink_supported && M::KIND == LatexModeKind::InnerParagraphMode {
            w.write_str(r"*")?;
        } else {
            w.write_char('}')?;
        }
        if is_hyperlink_supported {
            w.write_char('}')?;
        }
        Ok(())
    }
}