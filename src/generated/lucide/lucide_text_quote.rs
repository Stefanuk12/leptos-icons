use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 6H3" /> < path d = "M21 12H8" /> < path d = "M21 18H8" /> < path d = "M3 12v6" /> < / > } } pub const LucideTextQuote : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;