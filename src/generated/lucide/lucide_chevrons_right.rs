use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "13 17 18 12 13 7" /> < polyline points = "6 17 11 12 6 7" /> < / > } } pub const LucideChevronsRight : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;