use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M4 12h8" /> < path d = "M4 18V6" /> < path d = "M12 18V6" /> < path d = "m17 12 3-2v8" /> < / > } } pub const LucideHeading1 : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;