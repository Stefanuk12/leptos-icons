use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "13" r = "8" /> < path d = "M5 3 2 6" /> < path d = "m22 6-3-3" /> < path d = "M6.38 18.7 4 21" /> < path d = "M17.64 18.67 20 21" /> < path d = "m9 13 2 2 4-4" /> < / > } } pub const LucideAlarmClockCheck : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;