use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "4" /> < path d = "M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8" /> < / > } } pub const LucideAtSign : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;