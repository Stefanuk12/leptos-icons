use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17a10 10 0 0 0-20 0" /> < path d = "M6 17a6 6 0 0 1 12 0" /> < path d = "M10 17a2 2 0 0 1 4 0" /> < / > } } pub const LucideRainbow : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;