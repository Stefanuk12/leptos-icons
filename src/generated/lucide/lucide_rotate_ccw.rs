use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M3 2v6h6" /> < path d = "M3 13a9 9 0 1 0 3-7.7L3 8" /> < / > } } pub const LucideRotateCcw : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;