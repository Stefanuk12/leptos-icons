use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M12 3v18" /> < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M3 9h18" /> < path d = "M3 15h18" /> < / > } } pub const LucideTable : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;