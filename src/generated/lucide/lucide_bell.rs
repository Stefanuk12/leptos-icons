use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" /> < path d = "M13.73 21a2 2 0 0 1-3.46 0" /> < / > } } pub const LucideBell : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;