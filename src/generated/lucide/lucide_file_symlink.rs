use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v7" /> < polyline points = "14 2 14 8 20 8" /> < path d = "m10 18 3-3-3-3" /> < path d = "M4 18v-1a2 2 0 0 1 2-2h6" /> < / > } } pub const LucideFileSymlink : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;