macro_rules! decl_latex_flavors {
    ($($flavor:ident,)+) => {        
        pub enum LatexFlavorKind {
            $(
                $flavor,
            )+
        }

        pub trait LatexFlavor {}

        pub trait LatexFlavorKindExt {
            const KIND: LatexFlavorKind;
        }

        $(
            pub struct $flavor;

            impl LatexFlavor for $flavor {}

            impl LatexFlavorKindExt for $flavor {
                const KIND: LatexFlavorKind = LatexFlavorKind::$flavor;
            }
        )+
    };
}

decl_latex_flavors!(
    //https://www.mediawiki.org/wiki/Texvc
    Texvc,
    PlainTex,
    AmsTex,
    AmsLatex,
    Latex209Plus,
    MathJax,
    Luatex,
    Xelatex,
);