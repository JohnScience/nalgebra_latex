use core::{fmt::Error, marker::PhantomData};

use crate::{
    latex_features::{LatexFeatures, NoFeatures},
    latex_flavors::{LatexFlavorKindExt, MathJax, AmsLatex},
    latex_modes::{DisplayMathMode, InlineMathMode, InnerParagraphMode, LatexMode}, fmt::labels::Label,
};

pub trait UnsafeWrite {
    unsafe fn write_str(&mut self, s: &str) -> Result<(), Error>;
    unsafe fn write_char(&mut self, c: char) -> core::fmt::Result;
    unsafe fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result;
}

pub trait WriteTwoDollarSignsTargetExt: LatexWriter {
    type WriteTwoDollarSignsTarget: LatexWriter<
        NestedWriter = Self::NestedWriter,
        Flavor = Self::Flavor,
        Features = Self::Features,
    >;
}

pub trait WriteDollarSignTargetExt: LatexWriter {
    type WriteDollarSignTarget: LatexWriter<
        NestedWriter = Self::NestedWriter,
        Flavor = Self::Flavor,
        Features = Self::Features,
    >
        // This doesn't seem to be deduced by the compiler but keeping it doesn't hurt
        + WriteDollarSignTargetExt<WriteDollarSignTarget = Self>;
}

pub trait WriteLabel: Sized + LatexWriter<Mode = DisplayMathMode> {
    fn write_label<L>(&mut self, label: &L) -> Result<(), Error>
    where
        L: Label;
}

pub trait LatexWriter: UnsafeWrite {
    type NestedWriter: core::fmt::Write;
    type Flavor: LatexFlavorKindExt;
    type Features: LatexFeatures;
    type Mode: LatexMode;

    // With GATs, the following types will be pointless in the trait.
    #[deprecated(
        since = "0.1.0",
        note = "This type together with other deprecated types in the trait is a temporary solution which
        soon will be swapped for one GAT."
    )]
    type InnerParagraphWriter: LatexWriter<
        Flavor = Self::Flavor,
        Features = Self::Features,
        Mode = InnerParagraphMode,
        NestedWriter = Self::NestedWriter,
    >;
    #[deprecated(
        since = "0.1.0",
        note = "This type together with other deprecated types in the trait is a temporary solution which
        soon will be swapped for one GAT."
    )]
    type DisplayMathWriter: LatexWriter<
        Flavor = Self::Flavor,
        Features = Self::Features,
        Mode = DisplayMathMode,
        NestedWriter = Self::NestedWriter,
    >;
    #[deprecated(
        since = "0.1.0",
        note = "This type together with other deprecated types in the trait is a temporary solution which
        soon will be swapped for one GAT."
    )]
    type InlineMathWriter: LatexWriter<
        Flavor = Self::Flavor,
        Features = Self::Features,
        Mode = InlineMathMode,
        NestedWriter = Self::NestedWriter,
    >;

    fn into_raw_parts(self) -> (Self::NestedWriter, Self::Features);
    unsafe fn from_raw_parts(w: Self::NestedWriter, features: Self::Features) -> Self;

    #[inline(always)]
    unsafe fn rebuild<T>(self) -> T
    where
        Self: Sized,
        T: LatexWriter<
            Flavor = Self::Flavor,
            Features = Self::Features,
            Mode = Self::Mode,
            NestedWriter = Self::NestedWriter,
        >,
    {
        let (nested_writer, features) = self.into_raw_parts();
        T::from_raw_parts(nested_writer, features)
    }

    #[inline(always)]
    fn new(w: Self::NestedWriter) -> Self
    where
        Self: Sized + LatexWriter<Features = NoFeatures>,
    {
        unsafe { Self::from_raw_parts(w, NoFeatures) }
    }

    fn default() -> Self
    where
        Self: Sized + LatexWriter<Features = NoFeatures>,
        Self::NestedWriter: Default,
    {
        Self::new(Default::default())
    }

    unsafe fn apply_to_nested_writer<F,O>(&mut self, f: F) -> O
    where
        F: Fn(&mut Self::NestedWriter) -> O;

    #[inline(always)]
    fn write_two_dollar_signs(mut self) -> Result<Self::WriteTwoDollarSignsTarget, Error>
    where
        Self: Sized + WriteTwoDollarSignsTargetExt,
    {
        unsafe { self.write_str("$$") }?;
        let (nested_writer, features) = self.into_raw_parts();
        Ok(unsafe { <_>::from_raw_parts(nested_writer, features) })
    }

    #[inline(always)]
    fn write_dollar_sign(mut self) -> Result<Self::WriteDollarSignTarget, Error>
    where
        Self: Sized + WriteDollarSignTargetExt,
    {
        unsafe { self.write_str("$") }?;
        let (nested_writer, features) = self.into_raw_parts();
        Ok(unsafe { <_>::from_raw_parts(nested_writer, features) })
    }
}

pub struct Writer<Fl, Fe, M, W> {
    writer: W,
    flavor: PhantomData<*const Fl>,
    features: Fe,
    mode: PhantomData<*const M>,
}

impl<Fl, Fe, M, W> Writer<Fl, Fe, M, W> {
    #[inline(always)]
    pub unsafe fn new(writer: W, features: Fe) -> Self {
        Self {
            writer,
            flavor: PhantomData,
            features,
            mode: PhantomData,
        }
    }
}

impl<Fl, Fe, M, W> Writer<Fl, Fe, M, W>
where
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
    M: LatexMode,
    W: core::fmt::Write,
{
    #[inline(always)]
    pub unsafe fn switch_mode_unchecked<Nm: LatexMode>(self) -> Writer<Fl, Fe, Nm, W> {
        Writer {
            writer: self.writer,
            flavor: self.flavor,
            features: self.features,
            mode: PhantomData,
        }
    }

    #[inline(always)]
    pub unsafe fn switch_features_unchecked<Nf>(self) -> Writer<Fl, Nf, M, W>
    where
        Nf: From<Fe>,
    {
        Writer {
            writer: self.writer,
            flavor: self.flavor,
            features: self.features.into(),
            mode: self.mode,
        }
    }
}

impl<Fl, Fe, M, W> UnsafeWrite for Writer<Fl, Fe, M, W>
where
    W: core::fmt::Write,
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
    M: LatexMode,
{
    #[inline(always)]
    unsafe fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.writer.write_str(s)
    }

    #[inline(always)]
    unsafe fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.writer.write_char(c)
    }

    #[inline(always)]
    unsafe fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        self.writer.write_fmt(args)
    }
}

impl<Fl, Fe, M, W> LatexWriter for Writer<Fl, Fe, M, W>
where
    W: core::fmt::Write,
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
    M: LatexMode,
{
    type NestedWriter = W;
    type Flavor = Fl;
    type Features = Fe;
    type Mode = M;

    type DisplayMathWriter = Writer<Fl, Fe, DisplayMathMode, W>;

    type InlineMathWriter = Writer<Fl, Fe, InlineMathMode, W>;

    type InnerParagraphWriter = Writer<Fl, Fe, InnerParagraphMode, W>;

    #[inline(always)]
    fn into_raw_parts(self) -> (Self::NestedWriter, Self::Features) {
        (self.writer, self.features)
    }

    #[inline(always)]
    unsafe fn from_raw_parts(w: Self::NestedWriter, features: Self::Features) -> Self {
        Self {
            writer: w,
            flavor: PhantomData,
            features,
            mode: PhantomData,
        }
    }

    unsafe fn apply_to_nested_writer<F,O>(&mut self, f: F) -> O
        where
            F: Fn(&mut Self::NestedWriter) -> O {
        f(&mut self.writer)
    }
}

impl<Fl, Fe, W> WriteTwoDollarSignsTargetExt for Writer<Fl, Fe, DisplayMathMode, W>
where
    W: core::fmt::Write,
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
{
    #[allow(deprecated)]
    type WriteTwoDollarSignsTarget = Self::InnerParagraphWriter;
}

impl<Fl, Fe, W> WriteTwoDollarSignsTargetExt for Writer<Fl, Fe, InnerParagraphMode, W>
where
    W: core::fmt::Write,
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
{
    #[allow(deprecated)]
    type WriteTwoDollarSignsTarget = Self::DisplayMathWriter;
}

impl<Fl, Fe, W> WriteDollarSignTargetExt for Writer<Fl, Fe, InlineMathMode, W>
where
    W: core::fmt::Write,
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
{
    #[allow(deprecated)]
    type WriteDollarSignTarget = Self::InnerParagraphWriter;
}

impl<Fl, Fe, W> WriteDollarSignTargetExt for Writer<Fl, Fe, InnerParagraphMode, W>
where
    W: core::fmt::Write,
    Fl: LatexFlavorKindExt,
    Fe: LatexFeatures,
{
    #[allow(deprecated)]
    type WriteDollarSignTarget = Self::InlineMathWriter;
}

impl<Fe,W> WriteLabel for Writer<MathJax, Fe, DisplayMathMode, W>
where
    Fe: LatexFeatures,
    W: core::fmt::Write,
{
    fn write_label<L>(&mut self, label: &L) -> Result<(), Error>
        where
        L: Label
    {
        if label.is_subeq() {
            // By default, MathJax doesn't support multiple labels so
            // we fall back to emulating them
            unsafe { self.write_str("& (") }?;
            unsafe { self.apply_to_nested_writer(|w| label.write_name(w)) }?;
            unsafe { self.write_char(')')}
        } else {
            unsafe { self.write_str(r"\tag{") }?;
            unsafe { self.apply_to_nested_writer(|w| label.write_name(w)) }?;
            unsafe { self.write_str("}") }?;
            unsafe { self.write_str(r"\label{") }?;
            unsafe { self.apply_to_nested_writer(|w| label.write_name(w)) }?;
            unsafe { self.write_str("}") }
        }
    }
}

impl<Fe,W> WriteLabel for Writer<AmsLatex, Fe, DisplayMathMode, W>
where
    Fe: LatexFeatures,
    W: core::fmt::Write,
{
    fn write_label<L>(&mut self, label: &L) -> Result<(), Error>
    where
        L: Label
    {
        unsafe { self.write_str(r"\tag{") }?;
        unsafe { self.apply_to_nested_writer(|w| label.write_name(w)) }?;
        unsafe { self.write_str("}") }?;
        unsafe { self.write_str(r"\label{") }?;
        unsafe { self.apply_to_nested_writer(|w| label.write_name(w)) }?;
        unsafe { self.write_str("}") }
    }
}