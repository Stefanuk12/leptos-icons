use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" /> < path d = "m10 8 4 4-4 4" /> < / > } } pub const LucideChevronRightCircle : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;