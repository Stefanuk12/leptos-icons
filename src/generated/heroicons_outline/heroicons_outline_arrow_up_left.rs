use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m19.5 19.5-15-15m0 0v11.25m0-11.25h11.25" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke-width" , "1.5")] } ;