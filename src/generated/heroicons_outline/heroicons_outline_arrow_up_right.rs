use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m4.5 19.5 15-15m0 0H8.25m11.25 0v11.25" /> < / > } } pub const HeroiconsOutlineArrowUpRight : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;