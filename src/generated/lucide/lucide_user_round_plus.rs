use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 13.292-6" /> < circle cx = "10" cy = "8" r = "5" /> < path d = "M19 16v6" /> < path d = "M22 19h-6" /> < / > } } pub const LucideUserRoundPlus : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;