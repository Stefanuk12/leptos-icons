use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /> < / > } } pub const LucideMoon : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;