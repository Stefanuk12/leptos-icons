use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3" /> < path d = "M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3" /> < line x1 = "4" x2 = "20" y1 = "12" y2 = "12" /> < / > } } pub const LucideSplitSquareVertical : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;