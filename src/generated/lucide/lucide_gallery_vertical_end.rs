use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M7 2h10" /> < path d = "M5 6h14" /> < rect width = "18" height = "12" x = "3" y = "10" rx = "2" /> < / > } } pub const LucideGalleryVerticalEnd : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;