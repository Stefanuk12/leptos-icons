use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "m9 8 6 4-6 4Z" /> < / > } } pub const LucidePlaySquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;