use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "20" x = "4" y = "2" rx = "2" /> < path d = "M12 6h.01" /> < circle cx = "12" cy = "14" r = "4" /> < path d = "M12 14h.01" /> < / > } } pub const LucideSpeaker : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;