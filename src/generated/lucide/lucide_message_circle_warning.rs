use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" /> < path d = "M12 8v4" /> < path d = "M12 16h.01" /> < / > } } pub const LucideMessageCircleWarning : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;