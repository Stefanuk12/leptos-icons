use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" /> < circle cx = "12" cy = "12" r = "2" /> < / > } } pub const LucideDisc : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;