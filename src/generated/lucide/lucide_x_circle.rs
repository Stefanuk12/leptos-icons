use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" /> < path d = "m15 9-6 6" /> < path d = "m9 9 6 6" /> < / > } } pub const LucideXCircle : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;