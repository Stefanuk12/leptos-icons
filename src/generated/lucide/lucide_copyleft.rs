use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" /> < path d = "M9.17 14.83a4 4 0 1 0 0-5.66" /> < / > } } pub const LucideCopyleft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;