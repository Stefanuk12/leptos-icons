use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3" /> < path d = "M8 21h8" /> < path d = "M12 17v4" /> < path d = "m17 8 5-5" /> < path d = "M17 3h5v5" /> < / > } } pub const LucideScreenShare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;