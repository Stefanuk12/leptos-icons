use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" /> < polyline points = "14 2 14 8 20 8" /> < path d = "m10 13-2 2 2 2" /> < path d = "m14 17 2-2-2-2" /> < / > } } pub const LucideFileCode : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;