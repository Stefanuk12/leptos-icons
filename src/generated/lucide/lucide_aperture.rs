use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" /> < path d = "m14.31 8 5.74 9.94" /> < path d = "M9.69 8h11.48" /> < path d = "m7.38 12 5.74-9.94" /> < path d = "M9.69 16 3.95 6.06" /> < path d = "M14.31 16H2.83" /> < path d = "m16.62 12-5.74 9.94" /> < / > } } pub const LucideAperture : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;