use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m4.5 4.5 15 15m0 0V8.25m0 11.25H8.25" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowDownRight : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;