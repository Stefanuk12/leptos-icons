use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "9" cy = "12" r = "1" /> < circle cx = "15" cy = "12" r = "1" /> < path d = "M8 20v2h8v-2" /> < path d = "m12.5 17-.5-1-.5 1h1z" /> < path d = "M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20" /> < / > } } pub const LucideSkull : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;