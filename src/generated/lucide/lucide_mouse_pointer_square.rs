use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" /> < path d = "m12 12 4 10 1.7-4.3L22 16Z" /> < / > } } pub const LucideMousePointerSquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;