use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "2" y = "2" width = "20" height = "8" rx = "2" ry = "2" /> < rect x = "2" y = "14" width = "20" height = "8" rx = "2" ry = "2" /> < line x1 = "6" y1 = "6" x2 = "6.01" y2 = "6" /> < line x1 = "6" y1 = "18" x2 = "6.01" y2 = "18" /> < / > } } pub const LucideServer : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;