use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m20 17-5-5 5-5" /> < path d = "m4 17 5-5-5-5" /> < / > } } pub const LucideChevronsRightLeft : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;