use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "10" r = "8" /> < circle cx = "12" cy = "10" r = "3" /> < path d = "M7 22h10" /> < path d = "M12 22v-4" /> < / > } } pub const LucideWebcam : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;