use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10" /> < path d = "M9 11h6" /> < path d = "M12 8v6" /> < / > } } pub const LucideShieldPlus : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;