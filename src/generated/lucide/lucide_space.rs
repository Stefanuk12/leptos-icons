use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" /> < / > } } pub const LucideSpace : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;