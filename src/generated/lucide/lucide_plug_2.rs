use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 2v6" /> < path d = "M15 2v6" /> < path d = "M12 17v5" /> < path d = "M5 8h14" /> < path d = "M6 11V8h12v3a6 6 0 1 1-12 0v0Z" /> < / > } } pub const LucidePlug2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;