use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 12h4l3 9 4-17h7" /> < / > } } pub const LucideRadical : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;