use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M3.75 6.75h16.5M3.75 12H12m-8.25 5.25h16.5" stroke - linejoin = "round" ></ path > < / > } } pub const HeroiconsOutlineBars3CenterLeft : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;