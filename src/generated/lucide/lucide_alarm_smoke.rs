use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 8a2 2 0 0 1-2-2V3h20v3a2 2 0 0 1-2 2Z" /> < path d = "m19 8-.8 3c-.1.6-.6 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L5 8" /> < path d = "M16 21c0-2.5 2-2.5 2-5" /> < path d = "M11 21c0-2.5 2-2.5 2-5" /> < path d = "M6 21c0-2.5 2-2.5 2-5" /> < / > } } pub const LucideAlarmSmoke : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;