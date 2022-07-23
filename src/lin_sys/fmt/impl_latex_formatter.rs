use nalgebra::{Dim, RawStorage};

use crate::{
    fmt::{LatexFormatter, PartialEndofunctionalWriteAsLatex},
    latex_features::LatexFeatures,
    latex_flavors::LatexFlavor,
    latex_modes::{MathLatexMode},
    lin_sys::{unknowns::Unknowns, LinSys},
};

use super::{CasesLinSysFormatter, PlainLinSysFormatter};

//use core::fmt::Write;
//
//use crate::{
//    env::LatexEnvironment,
//    fmt::{write_latex, LatexFormatter, WriteAsLatex},
//    latex_modes::{
//        CategorizedLatexModeKind, CategorizedLatexModeKindExt, CategoryEnumVariantExt,
//        ControlSeqDelimited, MathLatexMode, MathLatexModeKind,
//    },
//    lin_sys::{env::CasesEnvironment, unknowns::Unknowns, LinSys},
//};
//use nalgebra::{Dim, RawStorage};
//
//use super::{CasesLinSysFormatter, PlainLinSysFormatter};
//
   
impl<Fl, Fe, M, T, R, C, S, U> LatexFormatter<Fl, Fe, Fe, M, M, LinSys<T, R, C, S, U>>
    for PlainLinSysFormatter
where
    Fl: LatexFlavor,
    Fe: LatexFeatures,
    M: MathLatexMode,
    T: PartialEndofunctionalWriteAsLatex<Fl, Fe, M>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    U: Unknowns,
{
    fn write<IW, OW>(mut dest: IW, input: &LinSys<T, R, C, S, U>) -> Result<OW, core::fmt::Error>
    where
        IW: crate::latex_writer::LatexWriter<
            Flavor = Fl,
            Features = Fe,
            Mode = M,
            NestedWriter = OW::NestedWriter,
        >,
        OW: crate::latex_writer::LatexWriter<Flavor = Fl, Features = Fe, Mode = M>,
    {
        let nrows = input.matrix.nrows();
        let ncols = input.matrix.ncols();

        let ncols_sub2 = match ncols.checked_sub(2) {
            Some(n) => n,
            None => return Ok(unsafe { dest.rebuild() }),
        };
        for i in 0..nrows {
            for j in 0..ncols_sub2 {
                dest = input.matrix[(i, j)].partial_endofunctional_write_as_latex(dest)?;
                unsafe { input.unknowns.write_ith_unchecked(&mut dest, j) }?;
                unsafe { dest.write_char('+') }?;
            }
            dest = input.matrix[(i, ncols_sub2)].partial_endofunctional_write_as_latex(dest)?;
            unsafe { input.unknowns.write_ith_unchecked(&mut dest, ncols_sub2) }?;
            unsafe { dest.write_char('=') }?;
            dest = input.matrix[(i, ncols_sub2 + 1)].partial_endofunctional_write_as_latex(dest)?;
            if i != nrows - 1 {
                unsafe { dest.write_str(r"\\") }?;
            }
        }
        Ok(unsafe { dest.rebuild() })
    }
}

//impl<IM, OM, T, R, C, S, U> LatexFormatter<IM, OM, LinSys<T, R, C, S, U>> for CasesLinSysFormatter
//where
//    IM: CategorizedLatexModeKindExt,
//    OM: MathLatexMode
//        + CategoryEnumVariantExt<MathLatexModeKind>
//        + ControlSeqDelimited
//        + CategorizedLatexModeKindExt,
//    T: WriteAsLatex<OM>,
//    R: Dim,
//    C: Dim,
//    S: RawStorage<T, R, C>,
//    U: Unknowns,
//{
//    fn write_latex<W: core::fmt::Write>(
//        dest: &mut W,
//        input: &LinSys<T, R, C, S, U>,
//    ) -> Result<(), core::fmt::Error> {
//        use CategorizedLatexModeKind::*;
//        let is_delimiting_required = match IM::CATEGORIZED_KIND {
//            eq if eq == Math(OM::CATEGORY_ENUM_VARIANT) => Ok(false),
//            Math(_) => Err(core::fmt::Error),
//            _ => Ok(true),
//        }?;
//        if is_delimiting_required {
//            OM::write_opening_control_seq(dest)?;
//        }
//        CasesEnvironment::write_opening_tag(dest)?;
//        write_latex::<PlainLinSysFormatter, OM, OM, _, _>(dest, input)?;
//        CasesEnvironment::write_closing_tag(dest)?;
//        if is_delimiting_required {
//            OM::write_closing_control_seq(dest)?;
//        }
//        Ok(())
//    }
//}
