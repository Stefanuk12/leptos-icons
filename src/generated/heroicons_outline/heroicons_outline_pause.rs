use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.75 5.25v13.5m-7.5-13.5v13.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_PAUSE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24")] } ;