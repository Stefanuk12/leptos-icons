use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 15 4-8 4 8" /> < path d = "M4 13h6" /> < path d = "M15 11h4.5a2 2 0 0 1 0 4H15V7h4a2 2 0 0 1 0 4" /> < / > } } pub const LucideCaseUpper : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;