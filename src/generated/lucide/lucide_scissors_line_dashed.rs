use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.42 9.42 8 12" /> < circle cx = "4" cy = "8" r = "2" /> < path d = "m14 6-8.58 8.58" /> < circle cx = "4" cy = "16" r = "2" /> < path d = "M10.8 14.8 14 18" /> < path d = "M16 12h-2" /> < path d = "M22 12h-2" /> < / > } } pub const LucideScissorsLineDashed : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;