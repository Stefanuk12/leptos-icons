use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "14" height = "6" x = "5" y = "14" rx = "2" /> < rect width = "10" height = "6" x = "7" y = "4" rx = "2" /> < path d = "M2 20h20" /> < path d = "M2 10h20" /> < / > } } pub const LucideAlignVerticalDistributeEnd : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;