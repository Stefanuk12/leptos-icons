use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 14h-5" /> < path d = "M16 16v-3.5a2.5 2.5 0 0 1 5 0V16" /> < path d = "M4.5 13h6" /> < path d = "m3 16 4.5-9 4.5 9" /> < / > } } pub const LucideALargeSmall : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;