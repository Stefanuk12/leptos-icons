use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "11" cy = "11" r = "8" /> < line x1 = "21" y1 = "21" x2 = "16.65" y2 = "16.65" /> < / > } } pub const LucideSearch : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;