use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2Z" /> < path d = "m9 10 2 2 4-4" /> < / > } } pub const LucideBookmarkCheck : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;