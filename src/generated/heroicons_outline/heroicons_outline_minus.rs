use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M5 12h14" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("fill" , "none") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;