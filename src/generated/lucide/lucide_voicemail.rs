use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "6" cy = "12" r = "4" /> < circle cx = "18" cy = "12" r = "4" /> < line x1 = "6" x2 = "18" y1 = "16" y2 = "16" /> < / > } } pub const LucideVoicemail : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;