use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "12" cy = "12" r = "10" /> < path d = "M8 12h8" /> < path d = "m12 16 4-4-4-4" /> < / > } } pub const LucideArrowRightCircle : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;