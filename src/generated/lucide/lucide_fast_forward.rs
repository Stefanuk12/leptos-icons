use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polygon points = "13 19 22 12 13 5 13 19" /> < polygon points = "2 19 11 12 2 5 2 19" /> < / > } } pub const LucideFastForward : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;