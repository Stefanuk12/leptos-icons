use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" /> < path d = "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" /> < / > } } pub const LucideBookOpen : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;