use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 13V19H13" /> < path d = "M5 5L19 19" /> < / > } } pub const LucideMoveDownRight : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;