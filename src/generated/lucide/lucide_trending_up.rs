use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < polyline points = "22 7 13.5 15.5 8.5 10.5 2 17" /> < polyline points = "16 7 22 7 22 13" /> < / > } } pub const LucideTrendingUp : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;