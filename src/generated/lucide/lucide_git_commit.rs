use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "3" /> < line x1 = "3" y1 = "12" x2 = "9" y2 = "12" /> < line x1 = "15" y1 = "12" x2 = "21" y2 = "12" /> < / > } } pub const LucideGitCommit : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;