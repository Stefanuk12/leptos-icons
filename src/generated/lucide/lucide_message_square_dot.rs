use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M11.7 3H5a2 2 0 0 0-2 2v16l4-4h12a2 2 0 0 0 2-2v-2.7" /> < circle cx = "18" cy = "6" r = "3" /> < / > } } pub const LucideMessageSquareDot : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;