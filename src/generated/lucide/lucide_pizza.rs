use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M15 11h.01" /> < path d = "M11 15h.01" /> < path d = "M16 16h.01" /> < path d = "m2 16 20 6-6-20c-3.36.9-6.42 2.67-8.88 5.12A19.876 19.876 0 0 0 2 16Z" /> < path d = "M17 6c-6.29 1.47-9.43 5.13-11 11" /> < / > } } pub const LucidePizza : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;