use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M17 12h-2l-2 5-2-10-2 5H7" /> < / > } } pub const LucideActivitySquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;