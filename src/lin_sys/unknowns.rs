use crate::lin_sys::err::OutOfBoundsError;
use core::fmt::Write;
use nalgebra::Dim;
use either::Either;

pub trait Unknowns {
    fn write_latex<W: Write>(&self, w: &mut W) -> Result<(),core::fmt::Error>;
    fn validate_idx(&self, idx: usize) -> Result<(), OutOfBoundsError>;
    unsafe fn write_latex_for_ith_unchecked<W: Write>(&self, w: &mut W, zbi: usize) -> Result<(),core::fmt::Error>;
    // TODO: consider whether custom enum error type would be better
    fn write_latex_for_ith<W: Write>(&self, w: &mut W, zbi: usize) -> Result<(),Either<core::fmt::Error, OutOfBoundsError>> {
        self.validate_idx(zbi).map_err(Either::Right)?;
        unsafe { self.write_latex_for_ith_unchecked(w, zbi).map_err(Either::Left) }
    }
}

pub struct SingleLetterVecOfUnknowns<L>
{
    pub c: char,
    pub len: L,
}

impl<L> Unknowns for SingleLetterVecOfUnknowns<L>
where
    L: Copy + Dim
{
    fn write_latex<W: Write>(&self, w: &mut W) -> Result<(),core::fmt::Error> {
        w.write_fmt(format_args!("\\textbf{{{}}}", self.c))
    }

    fn validate_idx(&self, idx: usize) -> Result<(), OutOfBoundsError> {
        if idx >= self.len.value() {
            Err(OutOfBoundsError)
        } else {
            Ok(())
        }
    }

    unsafe fn write_latex_for_ith_unchecked<W: Write>(&self, w: &mut W, zbi: usize) -> Result<(),core::fmt::Error> {
        w.write_fmt(format_args!("{}_{{{}}}", self.c, zbi))
    }
}