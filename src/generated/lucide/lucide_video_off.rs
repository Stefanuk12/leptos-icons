use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M10.66 6H14a2 2 0 0 1 2 2v2.34l1 1L22 8v8" /> < path d = "M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2l10 10Z" /> < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" /> < / > } } pub const LucideVideoOff : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;