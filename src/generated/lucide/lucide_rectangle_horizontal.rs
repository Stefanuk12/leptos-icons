use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "20" height = "12" x = "2" y = "6" rx = "2" /> < / > } } pub const LucideRectangleHorizontal : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;