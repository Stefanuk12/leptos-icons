use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LONG_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;