use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" cy = "5" r = "2" /> < circle cx = "5" cy = "19" r = "2" /> < path d = "M5 17A12 12 0 0 1 17 5" /> < / > } } pub const LucideSpline : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;