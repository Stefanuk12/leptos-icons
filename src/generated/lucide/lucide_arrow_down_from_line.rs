use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 3H5" /> < path d = "M12 21V7" /> < path d = "m6 15 6 6 6-6" /> < / > } } pub const LucideArrowDownFromLine : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;