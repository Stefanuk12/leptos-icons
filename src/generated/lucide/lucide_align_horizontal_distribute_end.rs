use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "6" height = "14" x = "4" y = "5" rx = "2" /> < rect width = "6" height = "10" x = "14" y = "7" rx = "2" /> < path d = "M10 2v20" /> < path d = "M20 2v20" /> < / > } } pub const LucideAlignHorizontalDistributeEnd : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;