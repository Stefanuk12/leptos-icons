use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m19.5 19.5-15-15m0 0v11.25m0-11.25h11.25" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;