use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" /> < circle cx = "10" cy = "16" r = "2" /> < path d = "m16 10-4.5 4.5" /> < path d = "m15 11 1 1" /> < / > } } pub const LucideFileKey : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;