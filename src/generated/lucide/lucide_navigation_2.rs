use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < polygon points = "12 2 19 21 12 17 5 21 12 2" /> < / > } } pub const LucideNavigation2 : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;