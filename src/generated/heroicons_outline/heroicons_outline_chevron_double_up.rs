use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 18.75 7.5-7.5 7.5 7.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < path stroke - linejoin = "round" d = "m4.5 12.75 7.5-7.5 7.5 7.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineChevronDoubleUp : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("fill" , "none") , ("data-slot" , "icon")] } ;