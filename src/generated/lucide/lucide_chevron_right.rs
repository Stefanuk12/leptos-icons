use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 18 6-6-6-6" /> < / > } } pub const LucideChevronRight : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;