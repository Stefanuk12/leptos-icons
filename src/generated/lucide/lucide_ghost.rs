use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 10h.01" /> < path d = "M15 10h.01" /> < path d = "M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z" /> < / > } } pub const LucideGhost : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;