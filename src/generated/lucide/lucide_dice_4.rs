use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" ry = "2" /> < path d = "M16 8h.01" /> < path d = "M8 8h.01" /> < path d = "M8 16h.01" /> < path d = "M16 16h.01" /> < / > } } pub const LucideDice4 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;