use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M17 14h.01" /> < path d = "M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14" /> < / > } } pub const LucideWallet2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;