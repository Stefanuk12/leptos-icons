use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" /> < path d = "M9 17V7h4a3 3 0 0 1 0 6H9" /> < / > } } pub const LucideParkingSquare : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;