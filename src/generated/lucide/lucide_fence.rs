use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 3 2 5v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" /> < path d = "M6 8h4" /> < path d = "M6 18h4" /> < path d = "m12 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" /> < path d = "M14 8h4" /> < path d = "M14 18h4" /> < path d = "m20 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" /> < / > } } pub const LucideFence : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;