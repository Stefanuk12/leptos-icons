use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 13.87A4 4 0 0 1 7.41 6a5.11 5.11 0 0 1 1.05-1.54 5 5 0 0 1 7.08 0A5.11 5.11 0 0 1 16.59 6 4 4 0 0 1 18 13.87V21H6Z" /> < line x1 = "6" x2 = "18" y1 = "17" y2 = "17" /> < / > } } pub const LucideChefHat : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;