use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "12" height = "20" x = "6" y = "2" rx = "2" /> < rect width = "20" height = "12" x = "2" y = "6" rx = "2" /> < / > } } pub const LucideRatio : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;