use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M4.5 10.5 12 3m0 0 7.5 7.5M12 3v18" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowUp : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;