use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 6 2 7" /> < path d = "M10 6h4" /> < path d = "m22 7-2-1" /> < rect width = "16" height = "16" x = "4" y = "3" rx = "2" /> < path d = "M4 11h16" /> < path d = "M8 15h.01" /> < path d = "M16 15h.01" /> < path d = "M6 19v2" /> < path d = "M18 21v-2" /> < / > } } pub const LucideBusFront : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;