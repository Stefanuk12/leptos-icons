use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;