use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 5H19V11" /> < path d = "M19 5L5 19" /> < / > } } pub const LucideMoveUpRight : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;