use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" x2 = "12" y1 = "3" y2 = "21" /> < polyline points = "8 8 4 12 8 16" /> < polyline points = "16 16 20 12 16 8" /> < / > } } pub const LucideSeparatorVertical : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;