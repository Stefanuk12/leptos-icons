use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M9 3v18" /> < path d = "M9 15h12" /> < / > } } pub const LucidePanelsLeftBottom : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;