use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20h.01" /> < / > } } pub const LucideSignalZero : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;