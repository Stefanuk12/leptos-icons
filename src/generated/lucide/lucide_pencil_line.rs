use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M12 20h9" /> < path d = "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" /> < path d = "m15 5 3 3" /> < / > } } pub const LucidePencilLine : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;