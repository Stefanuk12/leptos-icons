use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M18 12H6" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS_SMALL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true")] } ;