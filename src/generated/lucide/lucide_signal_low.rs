use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20h.01" /> < path d = "M7 20v-4" /> < / > } } pub const LucideSignalLow : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;