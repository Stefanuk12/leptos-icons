use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m18 15-6-6-6 6" /> < / > } } pub const LucideChevronUp : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;