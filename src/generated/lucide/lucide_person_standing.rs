use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "5" r = "1" /> < path d = "m9 20 3-6 3 6" /> < path d = "m6 8 6 2 6-2" /> < path d = "M12 10v4" /> < / > } } pub const LucidePersonStanding : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;