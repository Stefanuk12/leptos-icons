use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" /> < path d = "M7 4v16" /> < rect x = "15" y = "4" width = "4" height = "6" ry = "2" /> < path d = "M17 20v-6h-2" /> < path d = "M15 20h4" /> < / > } } pub const LucideArrowUp01 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;