use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" cy = "19" r = "2" /> < circle cx = "5" cy = "5" r = "2" /> < path d = "M5 7v12h12" /> < path d = "m5 19 6-6" /> < / > } } pub const LucideScale3D : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;