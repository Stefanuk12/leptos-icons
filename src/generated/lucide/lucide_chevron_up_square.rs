use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "m8 14 4-4 4 4" /> < / > } } pub const LucideChevronUpSquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;