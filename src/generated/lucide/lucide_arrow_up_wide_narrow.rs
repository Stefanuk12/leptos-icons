use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m3 8 4-4 4 4" /> < path d = "M7 4v16" /> < path d = "M11 12h10" /> < path d = "M11 16h7" /> < path d = "M11 20h4" /> < / > } } pub const LucideArrowUpWideNarrow : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;