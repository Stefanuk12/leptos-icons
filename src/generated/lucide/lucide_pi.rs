use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "9" x2 = "9" y1 = "4" y2 = "20" /> < path d = "M4 7c0-1.7 1.3-3 3-3h13" /> < path d = "M18 20c-1.7 0-3-1.3-3-3V4" /> < / > } } pub const LucidePi : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;