use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "18" x2 = "18" y1 = "20" y2 = "10" /> < line x1 = "12" x2 = "12" y1 = "20" y2 = "4" /> < line x1 = "6" x2 = "6" y1 = "20" y2 = "14" /> < / > } } pub const LucideBarChart2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;