use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M7.5 3v18" /> < path d = "M12 3v18" /> < path d = "M16.5 3v18" /> < / > } } pub const LucideColumns4 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;