use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M3.75 9h16.5m-16.5 6.75h16.5" ></ path > < / > } } pub const HeroiconsOutlineBars2 : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;