use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LONG_LEFT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("aria-hidden" , "true")] } ;