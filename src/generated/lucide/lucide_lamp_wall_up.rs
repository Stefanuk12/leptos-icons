use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M11 4h6l3 7H8l3-7Z" /> < path d = "M14 11v5a2 2 0 0 1-2 2H8" /> < path d = "M4 15h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H4v-6Z" /> < / > } } pub const LucideLampWallUp : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;