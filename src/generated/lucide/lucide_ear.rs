use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0" /> < path d = "M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4" /> < / > } } pub const LucideEar : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;