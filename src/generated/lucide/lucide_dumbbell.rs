use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m6.5 6.5 11 11" /> < path d = "m21 21-1-1" /> < path d = "m3 3 1 1" /> < path d = "m18 22 4-4" /> < path d = "m2 6 4-4" /> < path d = "m3 10 7-7" /> < path d = "m14 21 7-7" /> < / > } } pub const LucideDumbbell : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;