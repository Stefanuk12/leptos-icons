use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> < line x1 = "12" y1 = "8" x2 = "12" y2 = "16" /> < line x1 = "8" y1 = "12" x2 = "16" y2 = "12" /> < / > } } pub const LucidePlusSquare : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;