use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "7" x = "3" y = "3" rx = "1" /> < rect width = "7" height = "7" x = "3" y = "14" rx = "1" /> < rect width = "7" height = "7" x = "14" y = "14" rx = "1" /> < / > } } pub const LucideLayoutPanelTop : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;