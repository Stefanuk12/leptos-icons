use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 21-4.3-4.3c-1-1-1-2.5 0-3.4l9.6-9.6c1-1 2.5-1 3.4 0l5.6 5.6c1 1 1 2.5 0 3.4L13 21" /> < path d = "M22 21H7" /> < path d = "m5 11 9 9" /> < / > } } pub const LucideEraser : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;