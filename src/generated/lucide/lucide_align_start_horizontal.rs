use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "4" y = "6" width = "6" height = "16" rx = "2" /> < rect x = "14" y = "6" width = "6" height = "9" rx = "2" /> < path d = "M22 2H2" /> < / > } } pub const LucideAlignStartHorizontal : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;