use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m2 8 2 2-2 2 2 2-2 2" /> < path d = "m22 8-2 2 2 2-2 2 2 2" /> < rect width = "8" height = "14" x = "8" y = "5" rx = "1" /> < / > } } pub const LucideVibrate : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;