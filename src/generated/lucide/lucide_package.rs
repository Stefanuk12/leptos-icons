use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "m7.5 4.27 9 5.15" /> < path d = "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" /> < path d = "m3.3 7 8.7 5 8.7-5" /> < path d = "M12 22V12" /> < / > } } pub const LucidePackage : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;