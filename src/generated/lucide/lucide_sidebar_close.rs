use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> < path d = "M9 3v18" /> < path d = "m16 15-3-3 3-3" /> < / > } } pub const LucideSidebarClose : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;