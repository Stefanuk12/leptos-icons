use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M6 18 18 6M6 6l12 12" ></ path > < / > } } pub const HeroiconsOutlineXMark : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "1.5")] } ;