use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M15.75 5.25v13.5m-7.5-13.5v13.5" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PAUSE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;