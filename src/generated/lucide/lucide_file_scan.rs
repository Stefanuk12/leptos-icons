use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 10V7l-5-5H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h4" /> < path d = "M14 2v4a2 2 0 0 0 2 2h4" /> < path d = "M16 14a2 2 0 0 0-2 2" /> < path d = "M20 14a2 2 0 0 1 2 2" /> < path d = "M20 22a2 2 0 0 0 2-2" /> < path d = "M16 22a2 2 0 0 1-2-2" /> < / > } } pub const LucideFileScan : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;