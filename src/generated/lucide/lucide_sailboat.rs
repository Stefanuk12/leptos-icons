use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M22 18H2a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4Z" /> < path d = "M21 14 10 2 3 14h18Z" /> < path d = "M10 2v16" /> < / > } } pub const LucideSailboat : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;