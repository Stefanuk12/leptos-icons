use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 9 3-3 3 3" /> < path d = "M13 18H7a2 2 0 0 1-2-2V6" /> < path d = "m22 15-3 3-3-3" /> < path d = "M11 6h6a2 2 0 0 1 2 2v10" /> < / > } } pub const LucideRepeat2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;