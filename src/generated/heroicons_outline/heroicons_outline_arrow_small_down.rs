use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M12 4.5v15m0 0l6.75-6.75M12 19.5l-6.75-6.75" /> < / > } } pub const HeroiconsOutlineArrowSmallDown : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;