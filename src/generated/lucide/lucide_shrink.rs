use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m15 15 6 6m-6-6v4.8m0-4.8h4.8" /> < path d = "M9 19.8V15m0 0H4.2M9 15l-6 6" /> < path d = "M15 4.2V9m0 0h4.8M15 9l6-6" /> < path d = "M9 4.2V9m0 0H4.2M9 9 3 3" /> < / > } } pub const LucideShrink : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;