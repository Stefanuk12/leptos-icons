use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M15.75 5.25v13.5m-7.5-13.5v13.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_PAUSE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;