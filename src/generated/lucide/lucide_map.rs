use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polygon points = "3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21" /> < line x1 = "9" y1 = "3" x2 = "9" y2 = "18" /> < line x1 = "15" y1 = "6" x2 = "15" y2 = "21" /> < / > } } pub const LucideMap : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;