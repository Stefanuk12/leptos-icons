use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polyline points = "4 14 10 14 10 20" /> < polyline points = "20 10 14 10 14 4" /> < line x1 = "14" y1 = "10" x2 = "21" y2 = "3" /> < line x1 = "3" y1 = "21" x2 = "10" y2 = "14" /> < / > } } pub const LucideMinimize2 : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;