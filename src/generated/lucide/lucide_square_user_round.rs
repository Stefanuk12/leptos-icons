use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a6 6 0 0 0-12 0" /> < circle cx = "12" cy = "11" r = "4" /> < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < / > } } pub const LucideSquareUserRound : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;