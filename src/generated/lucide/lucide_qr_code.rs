use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < rect x = "3" y = "3" width = "5" height = "5" rx = "1" /> < rect x = "16" y = "3" width = "5" height = "5" rx = "1" /> < rect x = "3" y = "16" width = "5" height = "5" rx = "1" /> < path d = "M21 16h-3a2 2 0 0 0-2 2v3" /> < path d = "M21 21v.01" /> < path d = "M12 7v3a2 2 0 0 1-2 2H7" /> < path d = "M3 12h.01" /> < path d = "M12 3h.01" /> < path d = "M12 16v.01" /> < path d = "M16 12h1" /> < path d = "M21 12v.01" /> < path d = "M12 21v-1" /> < / > } } pub const LucideQrCode : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;