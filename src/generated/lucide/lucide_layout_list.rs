use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "7" height = "7" x = "3" y = "3" rx = "1" /> < rect width = "7" height = "7" x = "3" y = "14" rx = "1" /> < path d = "M14 4h7" /> < path d = "M14 9h7" /> < path d = "M14 15h7" /> < path d = "M14 20h7" /> < / > } } pub const LucideLayoutList : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;