use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18" stroke - linejoin = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowLongUp : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "none")] } ;