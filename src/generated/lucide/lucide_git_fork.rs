use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "18" r = "3" /> < circle cx = "6" cy = "6" r = "3" /> < circle cx = "18" cy = "6" r = "3" /> < path d = "M18 9v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V9" /> < path d = "M12 12v3" /> < / > } } pub const LucideGitFork : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;