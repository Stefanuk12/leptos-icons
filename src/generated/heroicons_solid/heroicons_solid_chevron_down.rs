use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M12.53 16.28a.75.75 0 0 1-1.06 0l-7.5-7.5a.75.75 0 0 1 1.06-1.06L12 14.69l6.97-6.97a.75.75 0 1 1 1.06 1.06l-7.5 7.5Z" clip - rule = "evenodd" /> < / > } } pub const HeroiconsSolidChevronDown : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Solid) , } ;