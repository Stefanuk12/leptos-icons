use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "4" y1 = "21" x2 = "4" y2 = "14" /> < line x1 = "4" y1 = "10" x2 = "4" y2 = "3" /> < line x1 = "12" y1 = "21" x2 = "12" y2 = "12" /> < line x1 = "12" y1 = "8" x2 = "12" y2 = "3" /> < line x1 = "20" y1 = "21" x2 = "20" y2 = "16" /> < line x1 = "20" y1 = "12" x2 = "20" y2 = "3" /> < line x1 = "2" y1 = "14" x2 = "6" y2 = "14" /> < line x1 = "10" y1 = "8" x2 = "14" y2 = "8" /> < line x1 = "18" y1 = "16" x2 = "22" y2 = "16" /> < / > } } pub const LucideSliders : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;