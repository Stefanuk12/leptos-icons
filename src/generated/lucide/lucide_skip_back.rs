use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < polygon points = "19 20 9 12 19 4 19 20" /> < line x1 = "5" x2 = "5" y1 = "19" y2 = "5" /> < / > } } pub const LucideSkipBack : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;