use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "2" y = "5" width = "20" height = "14" rx = "2" /> < line x1 = "2" y1 = "10" x2 = "22" y2 = "10" /> < / > } } pub const LucideCreditCard : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;