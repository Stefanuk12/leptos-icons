use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path (cx : Scope) -> Fragment { view ! { cx , < > < circle cx = "18" cy = "18" r = "3" /> < circle cx = "6" cy = "6" r = "3" /> < path d = "M18 6V5" /> < path d = "M18 11v-1" /> < line x1 = "6" y1 = "9" x2 = "6" y2 = "21" /> < / > } } pub const LucideGitPullRequestDraft : Path = Path { path : icon_path , icon_type : IconType::HeroIcons(crate::HeroIconsType::Mini) , } ;