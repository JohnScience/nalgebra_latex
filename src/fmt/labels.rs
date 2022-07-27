use core::{fmt::Error, num::NonZeroU8};

use crate::{latex_modes::DisplayMathMode, latex_writer::{LatexWriter, WriteLabel}};

pub trait Label {
    fn write_name<W>(&self, dest: &mut W) -> Result<(), Error>
    where
        W: core::fmt::Write;
}

pub trait LabelGenerator {
    type Label: Label;
    type Change;
    type Error;

    unsafe fn write_next_label<W>(
        &mut self,
        dest: &mut W,
        c: Self::Change,
    ) -> Result<Self::Label, Self::Error>
    where
        W: LatexWriter<Mode = DisplayMathMode> + WriteLabel;
}

pub trait EqChangeExt: LabelGenerator {
    const EQ_CHANGE: Self::Change;
}

#[derive(Debug)]
pub struct LabelGenerationError;

pub enum CountersLabelGenerationError {
    LabelGenerationError,
    FormattingError(Error),
}

pub struct Counters {
    equation: usize,
    subeq: Option<NonZeroU8>,
}

pub enum CountersChange {
    IncrementEquation,
    IncrementEquationAndAddSubeq,
    IncrementSubeq,
}

pub enum CountersLabel {
    Equation(String),
    Subeq(String),
}

impl Counters {
    pub fn new() -> Self {
        Self {
            equation: 0,
            subeq: None,
        }
    }
}

impl LabelGenerator for Counters {
    type Change = CountersChange;
    type Error = CountersLabelGenerationError;
    type Label = CountersLabel;

    unsafe fn write_next_label<W>(
        &mut self,
        dest: &mut W,
        c: Self::Change,
    ) -> Result<Self::Label, CountersLabelGenerationError>
    where
        W: LatexWriter<Mode = DisplayMathMode> + WriteLabel,
    {
        use CountersLabelGenerationError::*;

        let label = match c {
            CountersChange::IncrementEquation => {
                self.equation = self.equation.checked_add(1).ok_or(LabelGenerationError)?;
                self.subeq = None;
                let label = format!("{}", self.equation);
                CountersLabel::Equation(label)
            }
            CountersChange::IncrementSubeq => {
                let n = match self.subeq {
                    Some(n) => n.get(),
                    None => return Err(LabelGenerationError),
                };
                if n == 26 {
                    return Err(LabelGenerationError);
                }
                let label = format!("{}{}", self.equation, (b'a' + n) as char);
                self.subeq = Some( NonZeroU8::new_unchecked(n + 1) );
                CountersLabel::Subeq(label)
            }
            CountersChange::IncrementEquationAndAddSubeq => {
                self.equation = self.equation.checked_add(1).ok_or(LabelGenerationError)?;
                self.subeq = Some( NonZeroU8::new_unchecked(1) );
                let label = format!("{}a", self.equation);
                CountersLabel::Subeq(label)
            }
        };
        dest.write_label(&label).map_err(FormattingError)?;
        Ok(label)
    }
}

impl EqChangeExt for Counters {
    const EQ_CHANGE: Self::Change = CountersChange::IncrementEquation;
}

impl Label for CountersLabel {
    fn write_name<W>(&self, dest: &mut W) -> Result<(), Error>
    where
        W: core::fmt::Write,
    {
        match self {
            CountersLabel::Equation(label) => dest.write_str(label.as_str()),
            CountersLabel::Subeq(label) => dest.write_str(label.as_str()),
        }
    }
}
