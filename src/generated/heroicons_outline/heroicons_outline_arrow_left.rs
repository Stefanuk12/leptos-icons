use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true")] } ;