use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" /> < path d = "M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" /> < circle cx = "12" cy = "12" r = "3" /> < path d = "m18 22-3-3 3-3" /> < path d = "m6 2 3 3-3 3" /> < / > } } pub const LucideSwitchCamera : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;