use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m16 16 2 2 4-4" /> < path d = "M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" /> < path d = "M16.5 9.4 7.55 4.24" /> < polyline points = "3.29 7 12 12 20.71 7" /> < line x1 = "12" y1 = "22" x2 = "12" y2 = "12" /> < / > } } pub const LucidePackageCheck : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;