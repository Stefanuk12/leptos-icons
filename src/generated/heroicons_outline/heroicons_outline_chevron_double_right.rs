use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m5.25 4.5 7.5 7.5-7.5 7.5m6-15 7.5 7.5-7.5 7.5" /> < / > } } pub const HeroiconsOutlineChevronDoubleRight : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Outline) , } ;