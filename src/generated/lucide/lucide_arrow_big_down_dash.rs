use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 5H9" /> < path d = "M15 9v3h4l-7 7-7-7h4V9z" /> < / > } } pub const LucideArrowBigDownDash : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;