use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m14 5-3 3 2 7 8-8-7-2Z" /> < path d = "m14 5-3 3-3-3 3-3 3 3Z" /> < path d = "M9.5 6.5 4 12l3 6" /> < path d = "M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z" /> < / > } } pub const LucideLampDesk : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;