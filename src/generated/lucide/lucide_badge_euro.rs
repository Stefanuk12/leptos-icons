use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" /> < path d = "M7 12h5" /> < path d = "M15 9.4a4 4 0 1 0 0 5.2" /> < / > } } pub const LucideBadgeEuro : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;