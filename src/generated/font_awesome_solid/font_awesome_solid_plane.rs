use leptos :: * ; use crate :: { IconType , Path } ; fn icon_path () -> Fragment { view ! { < > <!--! Font Awesome Free 6.5 . 1 by @ fontawesome - https : < / > } } pub const FontAwesomeSolidPlane : Path = Path { path : icon_path , icon_type : IconType::FontAwesome(crate::FontAwesomeType::Solid) , } ;