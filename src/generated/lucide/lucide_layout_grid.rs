use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "3" width = "7" height = "7" /> < rect x = "14" y = "3" width = "7" height = "7" /> < rect x = "14" y = "14" width = "7" height = "7" /> < rect x = "3" y = "14" width = "7" height = "7" /> < / > } } pub const LucideLayoutGrid : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;