use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" /> < circle cx = "12" cy = "10" r = "3" /> < / > } } pub const LucideMapPin : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;