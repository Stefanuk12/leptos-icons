use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 12H6" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HeroiconsOutlineMinusSmall : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;