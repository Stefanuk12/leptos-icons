use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < line x1 = "19" x2 = "5" y1 = "5" y2 = "19" /> < circle cx = "6.5" cy = "6.5" r = "2.5" /> < circle cx = "17.5" cy = "17.5" r = "2.5" /> < / > } } pub const LucidePercent : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;