use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8L2 12L6 16" /> < path d = "M2 12H22" /> < / > } } pub const LucideMoveLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;