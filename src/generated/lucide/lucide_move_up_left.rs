use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M5 11V5H11" /> < path d = "M5 5L19 19" /> < / > } } pub const LucideMoveUpLeft : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;