use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polygon points = "11 5 6 9 2 9 2 15 6 15 11 19 11 5" /> < path d = "M15.54 8.46a5 5 0 0 1 0 7.07" /> < / > } } pub const LucideVolume1 : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;