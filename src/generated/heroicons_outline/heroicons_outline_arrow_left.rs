use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowLeft : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5")] } ;