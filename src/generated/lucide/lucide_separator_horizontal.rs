use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "3" y1 = "12" x2 = "21" y2 = "12" /> < polyline points = "8 8 12 4 16 8" /> < polyline points = "16 16 12 20 8 16" /> < / > } } pub const LucideSeparatorHorizontal : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;