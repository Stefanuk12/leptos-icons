use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" /> < / > } } pub const LucideMessageSquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;