use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" /> < path d = "m9 9 12-2" /> < circle cx = "6" cy = "18" r = "3" /> < circle cx = "18" cy = "16" r = "3" /> < / > } } pub const LucideMusic4 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;