use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "14 15 9 20 4 15" /> < path d = "M20 4h-7a4 4 0 0 0-4 4v12" /> < / > } } pub const LucideCornerLeftDown : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;