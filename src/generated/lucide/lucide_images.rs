use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 22H4a2 2 0 0 1-2-2V6" /> < path d = "m22 13-1.296-1.296a2.41 2.41 0 0 0-3.408 0L11 18" /> < circle cx = "12" cy = "8" r = "2" /> < rect width = "16" height = "16" x = "6" y = "2" rx = "2" /> < / > } } pub const LucideImages : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;