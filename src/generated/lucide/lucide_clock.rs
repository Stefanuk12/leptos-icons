use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" /> < polyline points = "12 6 12 12 16 14" /> < / > } } pub const LucideClock : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;