use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3v3a2 2 0 0 1-2 2H3" /> < path d = "M21 8h-3a2 2 0 0 1-2-2V3" /> < path d = "M3 16h3a2 2 0 0 1 2 2v3" /> < path d = "M16 21v-3a2 2 0 0 1 2-2h3" /> < / > } } pub const LucideMinimize : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;