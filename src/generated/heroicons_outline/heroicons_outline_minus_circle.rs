use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M15 12H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS_CIRCLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("data-slot" , "icon")] } ;