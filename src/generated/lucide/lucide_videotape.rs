use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "16" x = "2" y = "4" rx = "2" /> < path d = "M2 8h20" /> < circle cx = "8" cy = "14" r = "2" /> < path d = "M8 12h8" /> < circle cx = "16" cy = "14" r = "2" /> < / > } } pub const LucideVideotape : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;