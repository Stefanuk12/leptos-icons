use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 21V3h8" /> < path d = "M6 16h9" /> < path d = "M10 9.5h7" /> < / > } } pub const LucideSwissFranc : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;