use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M19.5 12h-15m0 0 6.75 6.75M4.5 12l6.75-6.75" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_SMALL_LEFT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;