use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M18 12H6" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS_SMALL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24")] } ;