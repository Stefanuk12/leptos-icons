use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" x2 = "21" y1 = "22" y2 = "22" /> < line x1 = "6" x2 = "6" y1 = "18" y2 = "11" /> < line x1 = "10" x2 = "10" y1 = "18" y2 = "11" /> < line x1 = "14" x2 = "14" y1 = "18" y2 = "11" /> < line x1 = "18" x2 = "18" y1 = "18" y2 = "11" /> < polygon points = "12 2 20 7 4 7" /> < / > } } pub const LucideLandmark : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;