use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m5 8 6 6" /> < path d = "m4 14 6-6 2-3" /> < path d = "M2 5h12" /> < path d = "M7 2h1" /> < path d = "m22 22-5-10-5 10" /> < path d = "M14 18h6" /> < / > } } pub const LucideLanguages : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;