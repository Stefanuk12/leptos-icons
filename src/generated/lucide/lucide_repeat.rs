use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m17 2 4 4-4 4" /> < path d = "M3 11v-1a4 4 0 0 1 4-4h14" /> < path d = "m7 22-4-4 4-4" /> < path d = "M21 13v1a4 4 0 0 1-4 4H3" /> < / > } } pub const LucideRepeat : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;