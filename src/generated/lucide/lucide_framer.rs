use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M5 16V9h14V2H5l14 14h-7m-7 0 7 7v-7m-7 0h7" /> < / > } } pub const LucideFramer : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;