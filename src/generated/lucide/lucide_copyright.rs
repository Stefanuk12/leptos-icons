use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "10" /> < path d = "M14.83 14.83a4 4 0 1 1 0-5.66" /> < / > } } pub const LucideCopyright : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;