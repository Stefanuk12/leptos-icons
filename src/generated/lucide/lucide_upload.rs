use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /> < polyline points = "17 8 12 3 7 8" /> < line x1 = "12" x2 = "12" y1 = "3" y2 = "15" /> < / > } } pub const LucideUpload : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;