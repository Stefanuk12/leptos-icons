use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 2 20 20" /> < path d = "M8.35 2.69A10 10 0 0 1 21.3 15.65" /> < path d = "M19.08 19.08A10 10 0 1 1 4.92 4.92" /> < / > } } pub const LucideCircleOff : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;