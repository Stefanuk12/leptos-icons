use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 14 4 9l5-5" /> < path d = "M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11" /> < / > } } pub const LucideUndo2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;