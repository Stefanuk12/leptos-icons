use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z" /> < / > } } pub const LucideCloud : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;