use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "4" /> < path d = "M12 2v2" /> < path d = "M12 20v2" /> < path d = "m4.93 4.93 1.41 1.41" /> < path d = "m17.66 17.66 1.41 1.41" /> < path d = "M2 12h2" /> < path d = "M20 12h2" /> < path d = "m6.34 17.66-1.41 1.41" /> < path d = "m19.07 4.93-1.41 1.41" /> < / > } } pub const LucideSun : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;