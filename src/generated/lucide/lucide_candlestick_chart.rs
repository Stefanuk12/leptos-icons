use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M9 5v4" /> < rect width = "4" height = "6" x = "7" y = "9" rx = "1" /> < path d = "M9 15v2" /> < path d = "M17 3v2" /> < rect width = "4" height = "8" x = "15" y = "5" rx = "1" /> < path d = "M17 13v3" /> < path d = "M3 3v18h18" /> < / > } } pub const LucideCandlestickChart : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;