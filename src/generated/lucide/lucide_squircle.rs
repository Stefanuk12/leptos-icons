use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3c7.2 0 9 1.8 9 9s-1.8 9-9 9-9-1.8-9-9 1.8-9 9-9" /> < / > } } pub const LucideSquircle : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;