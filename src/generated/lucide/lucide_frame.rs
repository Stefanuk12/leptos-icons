use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < line x1 = "22" x2 = "2" y1 = "6" y2 = "6" /> < line x1 = "22" x2 = "2" y1 = "18" y2 = "18" /> < line x1 = "6" x2 = "6" y1 = "2" y2 = "22" /> < line x1 = "18" x2 = "18" y1 = "2" y2 = "22" /> < / > } } pub const LucideFrame : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;