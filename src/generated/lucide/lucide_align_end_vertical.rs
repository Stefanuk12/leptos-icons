use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "2" y = "4" width = "16" height = "6" rx = "2" /> < rect x = "9" y = "14" width = "9" height = "6" rx = "2" /> < path d = "M22 22V2" /> < / > } } pub const LucideAlignEndVertical : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;