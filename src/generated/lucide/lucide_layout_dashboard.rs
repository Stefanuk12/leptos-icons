use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect width = "7" height = "9" x = "3" y = "3" rx = "1" /> < rect width = "7" height = "5" x = "14" y = "3" rx = "1" /> < rect width = "7" height = "9" x = "14" y = "12" rx = "1" /> < rect width = "7" height = "5" x = "3" y = "16" rx = "1" /> < / > } } pub const LucideLayoutDashboard : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;