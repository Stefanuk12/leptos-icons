use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M15 12H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS_CIRCLE : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;