use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M10 10 4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-1.272-2.542" /> < path d = "M10 2v2.343" /> < path d = "M14 2v6.343" /> < path d = "M8.5 2h7" /> < path d = "M7 16h9" /> < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" /> < / > } } pub const LucideFlaskConicalOff : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;