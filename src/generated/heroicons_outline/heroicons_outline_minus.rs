use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M5 12h14" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("stroke" , "currentColor")] } ;