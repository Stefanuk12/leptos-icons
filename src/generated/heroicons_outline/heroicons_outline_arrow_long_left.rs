use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowLongLeft : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("aria-hidden" , "true")] } ;