use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "16 18 22 12 16 6" /> < polyline points = "8 6 2 12 8 18" /> < / > } } pub const LucideCode : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;