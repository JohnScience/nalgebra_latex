pub mod latex_modes {
    pub enum LatexModeCategory {
        Horizontal,
        Math,
        Vertical,
    }
    impl ::core::marker::StructuralPartialEq for LatexModeCategory {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for LatexModeCategory {
        #[inline]
        fn eq(&self, other: &LatexModeCategory) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for LatexModeCategory {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for LatexModeCategory {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    pub enum LatexModeKind {
        OuterParagraphMode,
        InnerParagraphMode,
        LRMode,
        InlineMathMode,
        DisplayMathMode,
        VerticalMode,
        InternalVerticalMode,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LatexModeKind {
        #[inline]
        fn clone(&self) -> LatexModeKind {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for LatexModeKind {}
    impl ::core::marker::StructuralPartialEq for LatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for LatexModeKind {
        #[inline]
        fn eq(&self, other: &LatexModeKind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for LatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for LatexModeKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    pub enum CategorizedLatexModeKind {
        Horizontal(HorizontalLatexModeKind),
        Math(MathLatexModeKind),
        Vertical(VerticalLatexModeKind),
    }
    impl ::core::marker::StructuralPartialEq for CategorizedLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for CategorizedLatexModeKind {
        #[inline]
        fn eq(&self, other: &CategorizedLatexModeKind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &CategorizedLatexModeKind::Horizontal(ref __self_0),
                            &CategorizedLatexModeKind::Horizontal(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        (
                            &CategorizedLatexModeKind::Math(ref __self_0),
                            &CategorizedLatexModeKind::Math(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        (
                            &CategorizedLatexModeKind::Vertical(ref __self_0),
                            &CategorizedLatexModeKind::Vertical(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &CategorizedLatexModeKind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &CategorizedLatexModeKind::Horizontal(ref __self_0),
                            &CategorizedLatexModeKind::Horizontal(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        (
                            &CategorizedLatexModeKind::Math(ref __self_0),
                            &CategorizedLatexModeKind::Math(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        (
                            &CategorizedLatexModeKind::Vertical(ref __self_0),
                            &CategorizedLatexModeKind::Vertical(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for CategorizedLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for CategorizedLatexModeKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<HorizontalLatexModeKind>;
                let _: ::core::cmp::AssertParamIsEq<MathLatexModeKind>;
                let _: ::core::cmp::AssertParamIsEq<VerticalLatexModeKind>;
            }
        }
    }
    pub trait LatexMode {
        fn kind(&self) -> LatexModeKind;
        fn categorized_kind(&self) -> CategorizedLatexModeKind {
            match self.kind() {
                LatexModeKind::OuterParagraphMode => CategorizedLatexModeKind::Horizontal(
                    HorizontalLatexModeKind::OuterParagraphMode,
                ),
                LatexModeKind::InnerParagraphMode => CategorizedLatexModeKind::Horizontal(
                    HorizontalLatexModeKind::InnerParagraphMode,
                ),
                LatexModeKind::LRMode => {
                    CategorizedLatexModeKind::Horizontal(HorizontalLatexModeKind::LRMode)
                }
                LatexModeKind::InlineMathMode => {
                    CategorizedLatexModeKind::Math(MathLatexModeKind::InlineMathMode)
                }
                LatexModeKind::DisplayMathMode => {
                    CategorizedLatexModeKind::Math(MathLatexModeKind::DisplayMathMode)
                }
                LatexModeKind::VerticalMode => {
                    CategorizedLatexModeKind::Vertical(VerticalLatexModeKind::VerticalMode)
                }
                LatexModeKind::InternalVerticalMode => {
                    CategorizedLatexModeKind::Vertical(VerticalLatexModeKind::InternalVerticalMode)
                }
            }
        }
        fn category(&self) -> LatexModeCategory {
            match self.kind() {
                LatexModeKind::OuterParagraphMode
                | LatexModeKind::InnerParagraphMode
                | LatexModeKind::LRMode => LatexModeCategory::Horizontal,
                LatexModeKind::InlineMathMode | LatexModeKind::DisplayMathMode => {
                    LatexModeCategory::Math
                }
                LatexModeKind::VerticalMode | LatexModeKind::InternalVerticalMode => {
                    LatexModeCategory::Vertical
                }
            }
        }
    }
    pub trait LatexModeKindExt: LatexMode {
        const KIND: LatexModeKind;
    }
    pub trait CategorizedLatexModeKindExt: LatexModeKindExt {
        const CATEGORIZED_KIND: CategorizedLatexModeKind;
    }
    impl LatexMode for LatexModeKind {
        fn kind(&self) -> LatexModeKind {
            *self
        }
    }
    pub trait CategoryEnumVariantExt<CE> {
        const CATEGORY_ENUM_VARIANT: CE;
    }
    pub trait HorizontalLatexMode: LatexMode {
        fn category_enum(&self) -> HorizontalLatexModeKind;
    }
    pub enum HorizontalLatexModeKind {
        OuterParagraphMode,
        InnerParagraphMode,
        LRMode,
    }
    impl ::core::marker::StructuralPartialEq for HorizontalLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for HorizontalLatexModeKind {
        #[inline]
        fn eq(&self, other: &HorizontalLatexModeKind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for HorizontalLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for HorizontalLatexModeKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl LatexMode for HorizontalLatexModeKind {
        fn kind(&self) -> LatexModeKind {
            match *self {
                HorizontalLatexModeKind::OuterParagraphMode => LatexModeKind::OuterParagraphMode,
                HorizontalLatexModeKind::InnerParagraphMode => LatexModeKind::InnerParagraphMode,
                HorizontalLatexModeKind::LRMode => LatexModeKind::LRMode,
            }
        }
    }
    pub struct OuterParagraphMode;
    impl LatexMode for OuterParagraphMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::OuterParagraphMode
        }
    }
    impl LatexModeKindExt for OuterParagraphMode {
        const KIND: LatexModeKind = LatexModeKind::OuterParagraphMode;
    }
    impl CategorizedLatexModeKindExt for OuterParagraphMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Horizontal(HorizontalLatexModeKind::OuterParagraphMode);
    }
    impl CategoryEnumVariantExt<HorizontalLatexModeKind> for OuterParagraphMode {
        const CATEGORY_ENUM_VARIANT: HorizontalLatexModeKind =
            HorizontalLatexModeKind::OuterParagraphMode;
    }
    impl HorizontalLatexMode for OuterParagraphMode {
        fn category_enum(&self) -> HorizontalLatexModeKind {
            HorizontalLatexModeKind::OuterParagraphMode
        }
    }
    pub struct InnerParagraphMode;
    impl LatexMode for InnerParagraphMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::InnerParagraphMode
        }
    }
    impl LatexModeKindExt for InnerParagraphMode {
        const KIND: LatexModeKind = LatexModeKind::InnerParagraphMode;
    }
    impl CategorizedLatexModeKindExt for InnerParagraphMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Horizontal(HorizontalLatexModeKind::InnerParagraphMode);
    }
    impl CategoryEnumVariantExt<HorizontalLatexModeKind> for InnerParagraphMode {
        const CATEGORY_ENUM_VARIANT: HorizontalLatexModeKind =
            HorizontalLatexModeKind::InnerParagraphMode;
    }
    impl HorizontalLatexMode for InnerParagraphMode {
        fn category_enum(&self) -> HorizontalLatexModeKind {
            HorizontalLatexModeKind::InnerParagraphMode
        }
    }
    pub struct LRMode;
    impl LatexMode for LRMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::LRMode
        }
    }
    impl LatexModeKindExt for LRMode {
        const KIND: LatexModeKind = LatexModeKind::LRMode;
    }
    impl CategorizedLatexModeKindExt for LRMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Horizontal(HorizontalLatexModeKind::LRMode);
    }
    impl CategoryEnumVariantExt<HorizontalLatexModeKind> for LRMode {
        const CATEGORY_ENUM_VARIANT: HorizontalLatexModeKind = HorizontalLatexModeKind::LRMode;
    }
    impl HorizontalLatexMode for LRMode {
        fn category_enum(&self) -> HorizontalLatexModeKind {
            HorizontalLatexModeKind::LRMode
        }
    }
    pub trait MathLatexMode: LatexMode {
        fn category_enum(&self) -> MathLatexModeKind;
    }
    pub enum MathLatexModeKind {
        InlineMathMode,
        DisplayMathMode,
    }
    impl ::core::marker::StructuralPartialEq for MathLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MathLatexModeKind {
        #[inline]
        fn eq(&self, other: &MathLatexModeKind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for MathLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for MathLatexModeKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl LatexMode for MathLatexModeKind {
        fn kind(&self) -> LatexModeKind {
            match *self {
                MathLatexModeKind::InlineMathMode => LatexModeKind::InlineMathMode,
                MathLatexModeKind::DisplayMathMode => LatexModeKind::DisplayMathMode,
            }
        }
    }
    pub struct InlineMathMode;
    impl LatexMode for InlineMathMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::InlineMathMode
        }
    }
    impl LatexModeKindExt for InlineMathMode {
        const KIND: LatexModeKind = LatexModeKind::InlineMathMode;
    }
    impl CategorizedLatexModeKindExt for InlineMathMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Math(MathLatexModeKind::InlineMathMode);
    }
    impl CategoryEnumVariantExt<MathLatexModeKind> for InlineMathMode {
        const CATEGORY_ENUM_VARIANT: MathLatexModeKind = MathLatexModeKind::InlineMathMode;
    }
    impl MathLatexMode for InlineMathMode {
        fn category_enum(&self) -> MathLatexModeKind {
            MathLatexModeKind::InlineMathMode
        }
    }
    pub struct DisplayMathMode;
    impl LatexMode for DisplayMathMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::DisplayMathMode
        }
    }
    impl LatexModeKindExt for DisplayMathMode {
        const KIND: LatexModeKind = LatexModeKind::DisplayMathMode;
    }
    impl CategorizedLatexModeKindExt for DisplayMathMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Math(MathLatexModeKind::DisplayMathMode);
    }
    impl CategoryEnumVariantExt<MathLatexModeKind> for DisplayMathMode {
        const CATEGORY_ENUM_VARIANT: MathLatexModeKind = MathLatexModeKind::DisplayMathMode;
    }
    impl MathLatexMode for DisplayMathMode {
        fn category_enum(&self) -> MathLatexModeKind {
            MathLatexModeKind::DisplayMathMode
        }
    }
    pub trait VerticalLatexMode: LatexMode {
        fn category_enum(&self) -> VerticalLatexModeKind;
    }
    pub enum VerticalLatexModeKind {
        VerticalMode,
        InternalVerticalMode,
    }
    impl ::core::marker::StructuralPartialEq for VerticalLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for VerticalLatexModeKind {
        #[inline]
        fn eq(&self, other: &VerticalLatexModeKind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for VerticalLatexModeKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for VerticalLatexModeKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl LatexMode for VerticalLatexModeKind {
        fn kind(&self) -> LatexModeKind {
            match *self {
                VerticalLatexModeKind::VerticalMode => LatexModeKind::VerticalMode,
                VerticalLatexModeKind::InternalVerticalMode => LatexModeKind::InternalVerticalMode,
            }
        }
    }
    pub struct VerticalMode;
    impl LatexMode for VerticalMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::VerticalMode
        }
    }
    impl LatexModeKindExt for VerticalMode {
        const KIND: LatexModeKind = LatexModeKind::VerticalMode;
    }
    impl CategorizedLatexModeKindExt for VerticalMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Vertical(VerticalLatexModeKind::VerticalMode);
    }
    impl CategoryEnumVariantExt<VerticalLatexModeKind> for VerticalMode {
        const CATEGORY_ENUM_VARIANT: VerticalLatexModeKind = VerticalLatexModeKind::VerticalMode;
    }
    impl VerticalLatexMode for VerticalMode {
        fn category_enum(&self) -> VerticalLatexModeKind {
            VerticalLatexModeKind::VerticalMode
        }
    }
    pub struct InternalVerticalMode;
    impl LatexMode for InternalVerticalMode {
        fn kind(&self) -> LatexModeKind {
            LatexModeKind::InternalVerticalMode
        }
    }
    impl LatexModeKindExt for InternalVerticalMode {
        const KIND: LatexModeKind = LatexModeKind::InternalVerticalMode;
    }
    impl CategorizedLatexModeKindExt for InternalVerticalMode {
        const CATEGORIZED_KIND: CategorizedLatexModeKind =
            CategorizedLatexModeKind::Vertical(VerticalLatexModeKind::InternalVerticalMode);
    }
    impl CategoryEnumVariantExt<VerticalLatexModeKind> for InternalVerticalMode {
        const CATEGORY_ENUM_VARIANT: VerticalLatexModeKind =
            VerticalLatexModeKind::InternalVerticalMode;
    }
    impl VerticalLatexMode for InternalVerticalMode {
        fn category_enum(&self) -> VerticalLatexModeKind {
            VerticalLatexModeKind::InternalVerticalMode
        }
    }
}
