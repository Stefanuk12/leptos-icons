use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 6-4-4-4 4" /> < path d = "M12 2v8" /> < rect width = "20" height = "8" x = "2" y = "14" rx = "2" /> < path d = "M6 18h.01" /> < path d = "M10 18h.01" /> < / > } } pub const LucideHardDriveUpload : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;