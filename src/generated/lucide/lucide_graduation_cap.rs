use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < path d = "M22 10v6M2 10l10-5 10 5-10 5z" /> < path d = "M6 12v5c3 3 9 3 12 0v-5" /> < / > } } pub const LucideGraduationCap : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;