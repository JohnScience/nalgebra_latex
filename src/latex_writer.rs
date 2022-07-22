use core::marker::PhantomData;

use crate::{
    latex_features::{LatexFeatures, NoFeatures},
    latex_flavors::LatexFlavorKindExt,
    latex_modes::LatexMode,
};

#[inline(always)]
pub fn write_latex<IW, OW, T>(w: IW, t: &T) -> Result<OW, core::fmt::Error>
where
    (IW, OW, T): WriteLatexTuple<IWriter = IW, OWriter = OW, Writable = T>,
{
    <(IW, OW, T) as WriteLatexTuple>::write_latex(w, t)
}

pub trait WriteLatexTuple {
    type IWriter;
    type OWriter;
    type Writable;

    fn write_latex(w: Self::IWriter, t: &Self::Writable)
        -> Result<Self::OWriter, core::fmt::Error>;
}

pub trait UnsafeWrite {
    unsafe fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error>;
    unsafe fn write_char(&mut self, c: char) -> core::fmt::Result;
    unsafe fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result;
}

pub trait LatexWriter: UnsafeWrite {
    type NestedWriter: core::fmt::Write;
    type Flavor: LatexFlavorKindExt;
    type Features: LatexFeatures;
    type Mode: LatexMode;

    fn to_raw_parts(self) -> (Self::NestedWriter, Self::Features);
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
        let (nested_writer, features) = self.to_raw_parts();
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
    unsafe fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
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

    #[inline(always)]
    fn to_raw_parts(self) -> (Self::NestedWriter, Self::Features) {
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
}
