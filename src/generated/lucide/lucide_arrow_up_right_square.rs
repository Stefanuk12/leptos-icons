use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M8 8h8v8" /> < path d = "m8 16 8-8" /> < / > } } pub const LucideArrowUpRightSquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;