use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "6" x2 = "6" y1 = "4" y2 = "20" /> < polygon points = "10,4 20,12 10,20" /> < / > } } pub const LucideStepForward : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;