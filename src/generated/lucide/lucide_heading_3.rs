use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M4 12h8" /> < path d = "M4 18V6" /> < path d = "M12 18V6" /> < path d = "M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2" /> < path d = "M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2" /> < / > } } pub const LucideHeading3 : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;