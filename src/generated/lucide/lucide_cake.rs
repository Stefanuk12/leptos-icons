use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8" /> < path d = "M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1" /> < path d = "M2 21h20" /> < path d = "M7 8v3" /> < path d = "M12 8v3" /> < path d = "M17 8v3" /> < path d = "M7 4h0.01" /> < path d = "M12 4h0.01" /> < path d = "M17 4h0.01" /> < / > } } pub const LucideCake : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;