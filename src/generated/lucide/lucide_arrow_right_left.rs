use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 3 4 4-4 4" /> < path d = "M20 7H4" /> < path d = "m8 21-4-4 4-4" /> < path d = "M4 17h16" /> < / > } } pub const LucideArrowRightLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;