use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M2 6h4" /> < path d = "M2 10h4" /> < path d = "M2 14h4" /> < path d = "M2 18h4" /> < rect width = "16" height = "20" x = "4" y = "2" rx = "2" /> < path d = "M9.5 8h5" /> < path d = "M9.5 12H16" /> < path d = "M9.5 16H14" /> < / > } } pub const LucideNotebookText : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;