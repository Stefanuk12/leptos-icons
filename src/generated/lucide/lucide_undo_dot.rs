use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "17" r = "1" /> < path d = "M3 7v6h6" /> < path d = "M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" /> < / > } } pub const LucideUndoDot : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;