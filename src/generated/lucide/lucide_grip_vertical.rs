use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "9" cy = "12" r = "1" /> < circle cx = "9" cy = "5" r = "1" /> < circle cx = "9" cy = "19" r = "1" /> < circle cx = "15" cy = "12" r = "1" /> < circle cx = "15" cy = "5" r = "1" /> < circle cx = "15" cy = "19" r = "1" /> < / > } } pub const LucideGripVertical : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;