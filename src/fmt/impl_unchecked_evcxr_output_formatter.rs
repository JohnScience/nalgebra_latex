use core::fmt::{Error, Write};
use super::{UncheckedEvcxrOutputFormatter, WriteFormated};
use mime_typed::MimeStrExt;

impl<I, T> UncheckedEvcxrOutputFormatter<I> for T
where
    T: WriteFormated<I>,
{
    unsafe fn write_evcxr_output_unchecked<M,W>(dest: &mut W, i: &I) -> Result<(), Error>
    where
        M: MimeStrExt,
        W: Write,
    {
        writeln!(dest, "EVCXR_BEGIN_CONTENT {}", M::MIME_STR)?;
        T::write_formated(dest, i)?;
        dest.write_str("\nEVCXR_END_CONTENT\n")
    }
}