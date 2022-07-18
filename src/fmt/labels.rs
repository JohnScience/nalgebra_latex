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
//! unsafe { label.eqref(&mut s) }.unwrap();
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

use core::fmt::{Error, Write};

pub struct Counters {
    pub equation: usize,
    others: core::marker::PhantomData<()>,
}

pub struct CountersLabel(String);

pub struct LabelGenerationError;

pub trait Label {
    unsafe fn eqref<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write;
    unsafe fn tag_n_label<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write;
}

pub trait LabelGenerator {
    type Label: Label;

    fn next_label(&mut self) -> Result<Self::Label, LabelGenerationError>;
}

impl Counters {
    pub fn new() -> Self {
        Self {
            equation: 1,
            others: core::marker::PhantomData,
        }
    }
}

impl Label for CountersLabel {
    unsafe fn eqref<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        w.write_str(r"$\eqref{")?;
        w.write_str(self.0.as_str())?;
        w.write_str("}$")
    }

    unsafe fn tag_n_label<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write
    {
        w.write_str(r"\tag{")?;
        w.write_str(self.0.as_str())?;
        w.write_char('}')?;
        w.write_str(r"\label{")?;
        w.write_str(self.0.as_str())?;
        w.write_char('}')
    }
}

impl LabelGenerator for Counters {
    type Label = CountersLabel;

    fn next_label(&mut self) -> Result<Self::Label, LabelGenerationError> {
        let label = format!("{}", self.equation);
        self.equation += 1;
        Ok(CountersLabel(label))
    }
}
