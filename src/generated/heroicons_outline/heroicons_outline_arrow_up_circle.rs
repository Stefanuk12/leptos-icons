use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m15 11.25-3-3m0 0-3 3m3-3v7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" /> < / > } } pub const HeroiconsOutlineArrowUpCircle : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;