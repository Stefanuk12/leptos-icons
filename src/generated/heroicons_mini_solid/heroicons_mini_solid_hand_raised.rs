use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path fill - rule = "evenodd" d = "M11 2a1 1 0 1 0-2 0v6.5a.5.5 0 0 1-1 0V3a1 1 0 1 0-2 0v5.5a.5.5 0 0 1-1 0V5a1 1 0 1 0-2 0v7a7 7 0 1 0 14 0V8a1 1 0 1 0-2 0v3.5a.5.5 0 0 1-1 0V3a1 1 0 1 0-2 0v5.5a.5.5 0 0 1-1 0V2Z" clip - rule = "evenodd" /> < / > } } pub const HeroiconsMiniSolidHandRaised : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;