use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "8" x = "5" y = "2" rx = "2" /> < rect width = "20" height = "8" x = "2" y = "14" rx = "2" /> < path d = "M6 18h2" /> < path d = "M12 18h6" /> < / > } } pub const LucideComputer : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;