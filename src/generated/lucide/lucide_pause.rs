use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "4" height = "16" x = "6" y = "4" /> < rect width = "4" height = "16" x = "14" y = "4" /> < / > } } pub const LucidePause : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;