use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "3" /> < path d = "M3 7V5a2 2 0 0 1 2-2h2" /> < path d = "M17 3h2a2 2 0 0 1 2 2v2" /> < path d = "M21 17v2a2 2 0 0 1-2 2h-2" /> < path d = "M7 21H5a2 2 0 0 1-2-2v-2" /> < / > } } pub const LucideFocus : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;