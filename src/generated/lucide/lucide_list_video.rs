use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12H3" /> < path d = "M16 6H3" /> < path d = "M12 18H3" /> < path d = "m16 12 5 3-5 3v-6Z" /> < / > } } pub const LucideListVideo : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;