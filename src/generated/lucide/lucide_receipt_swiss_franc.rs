use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" /> < path d = "M10 17V7h5" /> < path d = "M10 11h4" /> < path d = "M8 15h5" /> < / > } } pub const LucideReceiptSwissFranc : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;