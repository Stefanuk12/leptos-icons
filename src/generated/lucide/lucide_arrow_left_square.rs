use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "m12 8-4 4 4 4" /> < path d = "M16 12H8" /> < / > } } pub const LucideArrowLeftSquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;