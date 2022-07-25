use nalgebra::{Dim, RawStorage};

use crate::{fmt::{FormatAsLabelledDisplayMathBlock, PartialEndofunctionalWriteAsLatex, labels::{LabelGenerator, EqChangeExt}}, latex_writer::{LatexWriter, WriteTwoDollarSigns, WriteTwoDollarSignsTargetExt}, lin_sys::{LinSys, unknowns::Unknowns}, latex_flavors::LatexFlavor, latex_features::LatexFeatures, latex_modes::{DisplayMathMode, InnerParagraphMode}};

use super::CasesLinSysFormatter;


impl<Fl,Fe,T,R,C,S,U> FormatAsLabelledDisplayMathBlock<Fl,Fe,LinSys<T,R,C,S,U>>
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
    fn format_as_labelled_display_math_block<G,IW,OW,L>(
        &self,
        dest: IW,
        label_gen: &mut G,
        input: &LinSys<T,R,C,S,U>,
    ) -> Result<OW, core::fmt::Error>
    where
        G: LabelGenerator<Label = L> + EqChangeExt,
        IW: LatexWriter<
            Flavor = Fl,
            Features = Fe,
            Mode = InnerParagraphMode,
            NestedWriter = OW::NestedWriter,
        > + WriteTwoDollarSigns + WriteTwoDollarSignsTargetExt<Mode = DisplayMathMode>,
        OW: LatexWriter<Flavor = Fl, Features = Fe, Mode = InnerParagraphMode>
    {
        //let dest = WriteTwoDollarSigns::write_two_dollar_signs(dest)?;
        //let label = unsafe { label_gen.write_next_label(&mut dest, G::EQ_CHANGE) }
        //    .map_err(|_| core::fmt::Error);

    }
}