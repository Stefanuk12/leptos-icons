use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M6 4v6a6 6 0 0 0 12 0V4" /> < line x1 = "4" y1 = "20" x2 = "20" y2 = "20" /> < / > } } pub const LucideUnderline : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;