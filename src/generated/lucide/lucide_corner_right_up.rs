use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "10 9 15 4 20 9" /> < path d = "M4 20h7a4 4 0 0 0 4-4V4" /> < / > } } pub const LucideCornerRightUp : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;