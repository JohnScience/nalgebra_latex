use crate::latex_flavors::{LatexFlavor, AmsLatex, MathJax};

mod impl_write_as_latex;

pub trait SupportedFlavor: LatexFlavor {
    #[allow(deprecated)]
    fn is_referencable<'a,'b>(label: &'a Hyperlink<'b>) -> bool;
}

#[cfg_attr(
    not(feature = "silence_deprecation"),
    deprecated(since = "0.1.0", note = r"The implementation is a fallback to \textit{text}")
)]
pub struct Hyperlink<'a> {
    /// e.g. `hadamard_product`
    #[allow(dead_code)]
    referent: &'a str,
    /// e.g. `Hadamard product`
    reference: &'a str,
}

#[allow(deprecated)]
impl<'a> Hyperlink<'a> {
    pub fn new(referent: &'a str, reference: &'a str) -> Self {
        Self {
            referent,
            reference,
        }
    }
}

impl SupportedFlavor for AmsLatex {
    #[allow(deprecated)]
    fn is_referencable<'a,'b>(_label: &'a Hyperlink<'b>) -> bool {
        todo!()
    }
}

impl SupportedFlavor for MathJax {
    #[allow(deprecated)]
    fn is_referencable<'a,'b>(_label: &'a Hyperlink<'b>) -> bool {
        todo!()
    }
}