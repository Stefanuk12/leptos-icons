use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m3 15 4-8 4 8" /> < path d = "M4 13h6" /> < circle cx = "18" cy = "12" r = "3" /> < path d = "M21 9v6" /> < / > } } pub const LucideCaseSensitive : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;