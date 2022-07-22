use core::{fmt::Error, marker::PhantomData};

use nalgebra::Dim;

use super::err::OutOfBoundsError;
use crate::{
    latex_modes::MathLatexMode,
    latex_writer::LatexWriter,
    lin_sys::numbering::{Numbering, NumberingTy},
};

pub trait Unknowns {
    fn is_empty(&self) -> bool;

    fn len(&self) -> Result<usize, OutOfBoundsError>;

    fn validate_idx(&self, zbi: usize) -> Result<(), OutOfBoundsError>;

    fn write<W, M>(&self, w: &mut W) -> Result<(), Error>
    where
        M: MathLatexMode,
        W: LatexWriter<Mode = M>;

    unsafe fn write_ith_unchecked<W, M>(&self, w: &mut W, zbi: usize) -> Result<(), Error>
    where
        M: MathLatexMode,
        W: LatexWriter<Mode = M>;

    fn write_ith<W, M>(&self, w: &mut W, zbi: usize) -> Result<(), Error>
    where
        M: MathLatexMode,
        W: LatexWriter<Mode = M>,
    {
        self.validate_idx(zbi).map_err(|_| Error)?;
        unsafe { self.write_ith_unchecked(w, zbi) }
    }
}

pub struct SingleLetterBoldfaceVecOfUnknowns<L, const N: NumberingTy> {
    /// The name of the vector of unknowns, e.g. **x**
    pub c: char,
    /// The length of the vector of unknowns
    pub len: L,
    // the private field forbids the usage of "default contructor"
    phantom: PhantomData<()>,
}

impl<L, const N: NumberingTy> SingleLetterBoldfaceVecOfUnknowns<L, N> {
    #[cfg_attr(not(feature = "adt_const_params"), allow(non_upper_case_globals))]
    pub fn new(c: char, len: L) -> Self {
        use Numbering::*;
        debug_assert!(matches!(N, ZeroBased | OneBased));
        Self {
            c,
            len,
            phantom: PhantomData::<()>,
        }
    }
}

impl<L, const N: NumberingTy> Unknowns for SingleLetterBoldfaceVecOfUnknowns<L, N>
where
    L: Copy + Dim,
{
    fn is_empty(&self) -> bool {
        self.len.value() == 0
    }

    fn len(&self) -> Result<usize, OutOfBoundsError> {
        Ok(self.len.value())
    }

    fn write<W, M>(&self, w: &mut W) -> Result<(), core::fmt::Error>
    where
        M: MathLatexMode,
        W: LatexWriter<Mode = M>,
    {
        unsafe {
            w.write_str(r"\textbf{")?;
            w.write_char(self.c)?;
            w.write_str(r"}")
        }
    }

    fn validate_idx(&self, zbi: usize) -> Result<(), OutOfBoundsError> {
        if zbi >= self.len.value() {
            Err(OutOfBoundsError)
        } else {
            Ok(())
        }
    }

    #[cfg_attr(not(feature = "adt_const_params"), allow(non_upper_case_globals))]
    unsafe fn write_ith_unchecked<W, M>(&self, w: &mut W, zbi: usize) -> Result<(), Error>
    where
        M: MathLatexMode,
        W: LatexWriter<Mode = M>,
    {
        use Numbering::*;

        w.write_fmt(format_args!(
            "{}_{{{}}}",
            self.c,
            match N {
                ZeroBased => zbi,
                OneBased => zbi + 1,
                #[cfg_attr(feature = "adt_const_params", allow(unreachable_patterns))]
                _ => panic!("unsupported numbering"),
            }
        ))
    }
}
