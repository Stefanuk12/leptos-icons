use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M3 15h18" /> < path d = "m9 10 3-3 3 3" /> < / > } } pub const LucidePanelBottomOpen : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;