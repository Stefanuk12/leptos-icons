use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < polygon points = "5 3 19 12 5 21 5 3" /> < / > } } pub const LucidePlay : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;