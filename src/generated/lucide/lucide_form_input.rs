use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "20" height = "12" x = "2" y = "6" rx = "2" /> < path d = "M12 12h.01" /> < path d = "M17 12h.01" /> < path d = "M7 12h.01" /> < / > } } pub const LucideFormInput : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;