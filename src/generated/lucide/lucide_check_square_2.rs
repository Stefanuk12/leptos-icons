use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "m9 12 2 2 4-4" /> < / > } } pub const LucideCheckSquare2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;