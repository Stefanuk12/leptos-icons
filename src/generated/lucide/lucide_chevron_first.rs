use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "m17 18-6-6 6-6" /> < path d = "M7 6v12" /> < / > } } pub const LucideChevronFirst : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;