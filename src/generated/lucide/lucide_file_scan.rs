use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M20 10V7.5L14.5 2H6a2 2 0 0 0-2 2v16c0 1.1.9 2 2 2h4.5" /> < polyline points = "14 2 14 8 20 8" /> < path d = "M16 22a2 2 0 0 1-2-2" /> < path d = "M20 22a2 2 0 0 0 2-2" /> < path d = "M20 14a2 2 0 0 1 2 2" /> < path d = "M16 14a2 2 0 0 0-2 2" /> < / > } } pub const LucideFileScan : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;