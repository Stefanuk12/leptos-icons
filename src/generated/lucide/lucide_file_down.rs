use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" /> < path d = "M14 2v4a2 2 0 0 0 2 2h4" /> < path d = "M12 18v-6" /> < path d = "m9 15 3 3 3-3" /> < / > } } pub const LucideFileDown : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;