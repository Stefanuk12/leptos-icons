use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m19.5 4.5-15 15m0 0h11.25m-11.25 0V8.25" ></ path > < / > } } pub const HeroiconsOutlineArrowDownLeft : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;