use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path fill - rule = "evenodd" d = "M15 10a.75.75 0 0 1-.75.75H7.612l2.158 1.96a.75.75 0 1 1-1.04 1.08l-3.5-3.25a.75.75 0 0 1 0-1.08l3.5-3.25a.75.75 0 1 1 1.04 1.08L7.612 9.25h6.638A.75.75 0 0 1 15 10Z" clip - rule = "evenodd" /> < / > } } pub const HeroiconsMiniSolidArrowSmallLeft : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;