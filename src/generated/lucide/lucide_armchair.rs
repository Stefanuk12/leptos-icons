use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3" /> < path d = "M3 11v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H7v-2a2 2 0 0 0-4 0Z" /> < path d = "M5 18v2" /> < path d = "M19 18v2" /> < / > } } pub const LucideArmchair : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;