use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M4.5 12h15m0 0-6.75-6.75M19.5 12l-6.75 6.75" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_SMALL_RIGHT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("aria-hidden" , "true")] } ;