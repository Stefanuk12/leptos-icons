use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M11 11h4" /> < path d = "M11 15h7" /> < path d = "M11 19h10" /> < path d = "M9 7 6 4 3 7" /> < path d = "M6 6v14" /> < / > } } pub const LucideSortAsc : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;