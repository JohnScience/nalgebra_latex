//! Module with types and traits for labels and referencing
//!
//! # Example
//!
//! ```
//! use nalgebra::{matrix, Const};
//! use nalgebra_latex::{
//!     lin_sys::{
//!         LinSys,
//!         unknowns::SingleLetterBoldfaceVecOfUnknowns,
//!         numbering::Numbering,
//!         fmt::CasesLinSysFormatter,
//!     },
//!     fmt::{
//!         WriteLabelledDisplayMathBlock,
//!         labels::{
//!             Counters,
//!             Label,
//!         }
//!     },
//!     latex_modes::InnerParagraphMode,
//!     latex_flavors::AmsLatex,
//! };
//! use std::io::{stdout, Write};
//!
//! let mut c = Counters::new();
//! let mut s = String::new();
//!
//! let m = matrix!(
//!     6,2,-8;
//!     3,1,-4;
//! );
//! let vec_of_unknowns = SingleLetterBoldfaceVecOfUnknowns::<_,{Numbering::OneBased}>::new('x', Const::<2>);
//! let ls = LinSys::new(m, vec_of_unknowns).unwrap();
//!
//! let label = CasesLinSysFormatter::write_labelled_display_math_block(&mut c, &mut s, &ls).unwrap();
//!
//! s += r#"
//!
//! Linear system "#;
//! unsafe { label.eqref::<AmsLatex,InnerParagraphMode,_>(&mut s) }.unwrap();
//! s += r#" corresponds to the "#;
//!
//! s += r#"\hyperlink{augmented_matrix}{\textit{augmented matrix}}"#;
//! s += r#" $(A \vert \overrightarrow{b})$ where \\
//! "#;
//!
//! assert_eq!(s,
//! r#"$$\begin{cases}6x_{1}+2x_{2}=-8\\3x_{1}+1x_{2}=-4\end{cases}\tag{1}\label{1}$$
//!
//! Linear system $\eqref{1}$ corresponds to the \hyperlink{augmented_matrix}{\textit{augmented matrix}} $(A \vert \overrightarrow{b})$ where \\
//! "#);
//!
//! ```

use core::{
    num::NonZeroU8,
    fmt::{Error, Write}
};

use crate::latex_modes::{ControlSeqDelimited, InlineMathMode, LatexModeKind, LatexModeKindExt};

pub struct Counters {
    pub equation: usize,
    pub subeq : Option<NonZeroU8>,
    others: core::marker::PhantomData<()>,
}

pub enum CountersLabel {
    Equation(String),
    Subeq(String),
}

pub enum CountersChange {
    IncrementEquation,
    IncrementEquationAndAddSubeq,
    IncrementSubeq,
}

#[derive(Debug)]
pub struct LabelGenerationError;

pub trait Label {
    fn eqref<F,IM, W>(&self, w: &mut W) -> Result<(), Error>
    where
        F: LabelsSupportedLatexFlavorsKindExt,
        IM: LatexModeKindExt,
        W: Write;
    unsafe fn tag_n_label<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write;
}

pub trait LabelGenerator {
    type Label: Label;
    type Change;

    fn next_label(&mut self, c: Self::Change) -> Result<Self::Label, LabelGenerationError>;
}

pub trait EqChangeExt: LabelGenerator {
    const EQ_CHANGE: Self::Change;
}

macro_rules! decl_labels_supported_latex_flavors {
    ($($flavor:ident,)+) => {
        pub enum LabelsSupportedLatexFlavorsKind {
            $($flavor),+
        }

        pub trait LabelsSupportedLatexFlavorsKindExt {
            const FLAVOR: LabelsSupportedLatexFlavorsKind;
        }

        $(
            impl LabelsSupportedLatexFlavorsKindExt for crate::latex_flavors::$flavor {
                const FLAVOR: LabelsSupportedLatexFlavorsKind = LabelsSupportedLatexFlavorsKind::$flavor;
            }
        )+
    };
}

decl_labels_supported_latex_flavors!(
    AmsLatex,
    MathJax,
);

impl CountersLabel {
    fn as_str(&self) -> &str {
        match self {
            CountersLabel::Equation(s) => s.as_str(),
            CountersLabel::Subeq(s) => s.as_str(),
        }
    }

    fn tag<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        match self {
            CountersLabel::Equation(_) => w.write_str(r"\tag{"),
            CountersLabel::Subeq(_) => w.write_str("& ("),
        }?;
        w.write_str(self.as_str())?;
        w.write_char(match self {
            CountersLabel::Equation(_) => '}',
            CountersLabel::Subeq(_) => ')',
        })
    }
}

impl Counters {
    pub fn new() -> Self {
        Self {
            equation: 0,
            subeq: None,
            others: core::marker::PhantomData,
        }
    }
}

impl Label for CountersLabel {
    fn eqref<F,IM, W>(&self, w: &mut W) -> Result<(), Error>
    where
        F: LabelsSupportedLatexFlavorsKindExt,
        IM: LatexModeKindExt,
        W: Write,
    {
        // eqref needs to be in math mode, so we need to switch to math mode
        let is_delimiting_required = match IM::KIND {
            LatexModeKind::InlineMathMode | LatexModeKind::DisplayMathMode => false,
            _ => true,
        };

        if is_delimiting_required {
            InlineMathMode::write_opening_control_seq(w)?;
        }

        match (F::FLAVOR, self) {
            (LabelsSupportedLatexFlavorsKind::MathJax, CountersLabel::Subeq(s)) => {
                w.write_str(r"(")?;
                w.write_str(s.as_str())?;
                w.write_str(")")?;
            },
            _ => {
                w.write_str(r"\eqref{")?;
                w.write_str(self.as_str())?;
                w.write_str("}")?;
            }
        }

        if is_delimiting_required {
            InlineMathMode::write_closing_control_seq(w)?;
        }
        Ok(())
    }

    unsafe fn tag_n_label<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        self.tag(w)?;
        w.write_str(r"\label{")?;
        w.write_str(self.as_str())?;
        w.write_char('}')
    }
}

impl LabelGenerator for Counters {
    type Label = CountersLabel;
    type Change = CountersChange;

    fn next_label(&mut self, c: Self::Change) -> Result<Self::Label, LabelGenerationError> {
        let label = match c {
            CountersChange::IncrementEquation => {
                self.equation += 1;
                self.subeq = None;
                CountersLabel::Equation(format!("{}", self.equation))
            },
            CountersChange::IncrementSubeq => {
                
                let n = match  self.subeq {
                    Some(n) => n.get(),
                    None => return Err(LabelGenerationError),
                };
                if n == 26 {
                    return Err(LabelGenerationError);
                }
                let label = format!("{}{}", self.equation, (b'a' + n) as char);
                self.subeq = Some(unsafe { NonZeroU8::new_unchecked(n + 1) });
                CountersLabel::Subeq(label)
            },
            CountersChange::IncrementEquationAndAddSubeq => {
                self.equation += 1;
                self.subeq = Some(unsafe { NonZeroU8::new_unchecked(1) });
                CountersLabel::Subeq(format!("{}a", self.equation))
            },
        };
        Ok(label)
    }
}

impl EqChangeExt for Counters {
    const EQ_CHANGE: Self::Change = CountersChange::IncrementEquation;
}
