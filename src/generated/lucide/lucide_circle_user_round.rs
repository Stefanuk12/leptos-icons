use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M18 20a6 6 0 0 0-12 0" /> < circle cx = "12" cy = "10" r = "4" /> < circle cx = "12" cy = "12" r = "10" /> < / > } } pub const LucideCircleUserRound : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;