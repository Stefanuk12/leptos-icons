use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M4 10a7.31 7.31 0 0 0 10 10Z" /> < path d = "m9 15 3-3" /> < path d = "M17 13a6 6 0 0 0-6-6" /> < path d = "M21 13A10 10 0 0 0 11 3" /> < / > } } pub const LucideSatelliteDish : Path = Path { path : icon_path , icon_type : IconType::Lucide , } ;