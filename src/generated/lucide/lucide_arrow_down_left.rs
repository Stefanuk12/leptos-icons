use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 7 7 17" /> < path d = "M17 17H7V7" /> < / > } } pub const LucideArrowDownLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;