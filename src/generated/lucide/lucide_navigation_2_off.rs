use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M9.31 9.31 5 21l7-4 7 4-1.17-3.17" /> < path d = "M14.53 8.88 12 2l-1.17 3.17" /> < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" /> < / > } } pub const LucideNavigation2Off : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;