use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3" ></ path > < / > } } pub const HeroiconsOutlineArrowLongDown : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;