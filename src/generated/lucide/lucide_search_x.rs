use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m13.5 8.5-5 5" /> < path d = "m8.5 8.5 5 5" /> < circle cx = "11" cy = "11" r = "8" /> < path d = "m21 21-4.3-4.3" /> < / > } } pub const LucideSearchX : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;