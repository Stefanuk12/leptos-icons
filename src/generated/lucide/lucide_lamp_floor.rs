use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M9 2h6l3 7H6l3-7Z" /> < path d = "M12 9v13" /> < path d = "M9 22h6" /> < / > } } pub const LucideLampFloor : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;