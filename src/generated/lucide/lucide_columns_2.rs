use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M12 3v18" /> < / > } } pub const LucideColumns2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;