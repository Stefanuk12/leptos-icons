use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 10c0-4.4-3.6-8-8-8s-8 3.6-8 8 3.6 8 8 8h8" /> < polyline points = "16 14 20 18 16 22" /> < / > } } pub const LucideIterationCcw : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;