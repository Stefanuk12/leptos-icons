use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 2v7.31" /> < path d = "M14 9.3V1.99" /> < path d = "M8.5 2h7" /> < path d = "M14 9.3a6.5 6.5 0 1 1-4 0" /> < path d = "M5.52 16h12.96" /> < / > } } pub const LucideFlaskRound : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;