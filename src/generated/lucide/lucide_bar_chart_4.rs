use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" /> < path d = "M13 17V9" /> < path d = "M18 17V5" /> < path d = "M8 17v-3" /> < / > } } pub const LucideBarChart4 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;