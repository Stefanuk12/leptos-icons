use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "20" height = "12" x = "2" y = "6" rx = "6" ry = "6" /> < circle cx = "16" cy = "12" r = "2" /> < / > } } pub const LucideToggleRight : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;