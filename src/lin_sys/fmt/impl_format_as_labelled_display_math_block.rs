use nalgebra::{Dim, RawStorage};

use crate::{
    fmt::{
        labels::{EqChangeExt, LabelGenerator},
        FormatAsLabelledDisplayMathBlock, PartialEndofunctionalWriteAsLatex, LatexFormatter,
    },
    latex_features::LatexFeatures,
    latex_flavors::LatexFlavor,
    latex_modes::{DisplayMathMode, InnerParagraphMode},
    latex_writer::{LatexWriter, WriteTwoDollarSigns, WriteTwoDollarSignsTargetExt},
    lin_sys::{unknowns::Unknowns, LinSys},
};

use super::CasesLinSysFormatter;

impl<Fl, Fe, T, R, C, S, U> FormatAsLabelledDisplayMathBlock<Fl, Fe, LinSys<T, R, C, S, U>>
    for CasesLinSysFormatter
where
    Fl: LatexFlavor,
    Fe: LatexFeatures,
    T: PartialEndofunctionalWriteAsLatex<Fl, Fe, DisplayMathMode>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    U: Unknowns,
{
    #[allow(deprecated)]
    fn format_as_labelled_display_math_block<G, IW, OW, L>(
        dest: IW,
        label_gen: &mut G,
        input: &LinSys<T, R, C, S, U>,
    ) -> Result<(OW,L), core::fmt::Error>
    where
        G: LabelGenerator<Label = L> + EqChangeExt,
        IW: LatexWriter<
                Flavor = Fl,
                Features = Fe,
                Mode = InnerParagraphMode,
                NestedWriter = OW::NestedWriter,
            > + WriteTwoDollarSigns,
        IW::WriteTwoDollarSignsTarget: LatexWriter<Mode = DisplayMathMode>,
        IW::DisplayMathWriter: WriteTwoDollarSignsTargetExt<WriteTwoDollarSignsTarget = IW>,
        OW: LatexWriter<Flavor = Fl, Features = Fe, Mode = InnerParagraphMode>,
    {
        let dest = WriteTwoDollarSigns::write_two_dollar_signs(dest)?;
        #[allow(deprecated)]
        let mut dest: <IW as LatexWriter>::DisplayMathWriter
            = <Self as LatexFormatter<Fl,Fe,Fe,DisplayMathMode,DisplayMathMode,LinSys<T, R, C, S, U>>>::fmt(dest, input)?;
        let label = unsafe { label_gen.write_next_label(&mut dest, G::EQ_CHANGE) }
            .map_err(|_| core::fmt::Error)?;
        let dest = dest.write_two_dollar_signs()?;
        Ok((unsafe { dest.rebuild() }, label))
    }
}
