use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "5" y = "2" width = "14" height = "20" rx = "7" /> < path d = "M12 6v4" /> < / > } } pub const LucideMouse : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;