use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < polyline points = "8 18 12 22 16 18" /> < polyline points = "8 6 12 2 16 6" /> < line x1 = "12" x2 = "12" y1 = "2" y2 = "22" /> < / > } } pub const LucideMoveVertical : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;