use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 6L12 2L16 6" /> < path d = "M12 2V22" /> < / > } } pub const LucideMoveUp : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;