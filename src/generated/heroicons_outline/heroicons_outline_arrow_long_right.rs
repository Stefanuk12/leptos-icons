use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LONG_RIGHT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;