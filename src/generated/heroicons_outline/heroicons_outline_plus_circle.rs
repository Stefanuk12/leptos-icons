use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PLUS_CIRCLE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;