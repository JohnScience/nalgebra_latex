//use super::{CasesLinSysFormatter, PlainLinSysFormatter};
//use crate::{
//    fmt::{WriteAsLatex, WriteLabelledDisplayMathBlock},
//    latex_modes::DisplayMathMode,
//    lin_sys::{unknowns::Unknowns, LinSys},
//};
//use nalgebra::{Dim, RawStorage};
//
//macro_rules! decl_for_linsys_formatter {
//    ($formatter:ident) => {
//        impl<T, R, C, S, U> WriteLabelledDisplayMathBlock<LinSys<T, R, C, S, U>> for $formatter
//        where
//            T: WriteAsLatex<DisplayMathMode>,
//            R: Dim,
//            C: Dim,
//            S: RawStorage<T, R, C>,
//            U: Unknowns,
//        {
//        }
//    };
//}
//
//decl_for_linsys_formatter!(PlainLinSysFormatter);
//decl_for_linsys_formatter!(CasesLinSysFormatter);
