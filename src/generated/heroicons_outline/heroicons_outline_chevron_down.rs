use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m19.5 8.25-7.5 7.5-7.5-7.5" stroke - linecap = "round" ></ path > < / > } } pub const HeroiconsOutlineChevronDown : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true")] } ;