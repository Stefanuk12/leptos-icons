use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" /> < path d = "M7 20V4" /> < path d = "M11 4h4" /> < path d = "M11 8h7" /> < path d = "M11 12h10" /> < / > } } pub const LucideArrowDownNarrowWide : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;