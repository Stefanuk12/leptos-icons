use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 12H3" /> < path d = "m11 18 6-6-6-6" /> < path d = "M21 5v14" /> < / > } } pub const LucideArrowRightToLine : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;