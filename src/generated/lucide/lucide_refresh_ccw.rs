use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M3 2v6h6" /> < path d = "M21 12A9 9 0 0 0 6 5.3L3 8" /> < path d = "M21 22v-6h-6" /> < path d = "M3 12a9 9 0 0 0 15 6.7l3-2.7" /> < / > } } pub const LucideRefreshCcw : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;