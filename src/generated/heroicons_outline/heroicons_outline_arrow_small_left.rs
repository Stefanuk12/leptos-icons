use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M19.5 12h-15m0 0 6.75 6.75M4.5 12l6.75-6.75" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_SMALL_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;