use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" height = "10" x = "9" y = "7" rx = "2" /> < path d = "M4 22V2" /> < path d = "M20 22V2" /> < / > } } pub const LucideAlignHorizontalSpaceAround : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;