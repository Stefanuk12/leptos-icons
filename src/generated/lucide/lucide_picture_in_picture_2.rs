use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4" /> < rect width = "10" height = "7" x = "12" y = "13" rx = "2" /> < / > } } pub const LucidePictureInPicture2 : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;