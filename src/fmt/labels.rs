use core::{num::NonZeroU8, fmt::Error};

use crate::{latex_writer::LatexWriter, latex_modes::DisplayMathMode};

pub trait Label {}

pub trait LabelGenerator {
    type Label: Label;
    type Change;
    type Error;

    unsafe fn write_next_label<W>(
        &mut self,
        dest: &mut W,
        c: Self::Change
    ) -> Result<Self::Label, Self::Error>
    where
        W: LatexWriter<Mode = DisplayMathMode>;
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
    subeq: Option<NonZeroU8>
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

impl LabelGenerator for Counters {
    type Change = CountersChange;
    type Error = CountersLabelGenerationError;
    type Label = CountersLabel;

    unsafe fn write_next_label<W>(
        &mut self,
        dest: &mut W,
        c: Self::Change
    )-> Result<Self::Label, CountersLabelGenerationError>
    where
        W: LatexWriter<Mode = DisplayMathMode>,
    {
        use CountersLabelGenerationError::*;

        let label = match c {
            CountersChange::IncrementEquation => {
                self.equation = self.equation.checked_add(1).ok_or(LabelGenerationError)?;
                self.subeq = None;
                let label = format!("{}", self.equation);
                dest.write_str(label.as_str()).map_err(FormattingError)?;
                CountersLabel::Equation(label)
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
                dest.write_str(label.as_str()).map_err(FormattingError)?;
                self.subeq = Some(unsafe { NonZeroU8::new_unchecked(n + 1) });
                CountersLabel::Subeq(label)
            },
            CountersChange::IncrementEquationAndAddSubeq => {
                self.equation = self.equation.checked_add(1).ok_or(LabelGenerationError)?;
                self.subeq = Some(unsafe { NonZeroU8::new_unchecked(1) });
                let label = format!("{}a", self.equation);
                dest.write_str(label.as_str()).map_err(FormattingError)?;
                CountersLabel::Subeq(label)
            },
        };
        Ok(label)
    }
}

impl EqChangeExt for Counters {
    const EQ_CHANGE: Self::Change = CountersChange::IncrementEquation;
}

impl Label for CountersLabel {
}
