use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 18h8" /> < path d = "M3 22h18" /> < path d = "M14 22a7 7 0 1 0 0-14h-1" /> < path d = "M9 14h2" /> < path d = "M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z" /> < path d = "M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3" /> < / > } } pub const LucideMicroscope : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;