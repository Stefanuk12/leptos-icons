use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m19.5 4.5-15 15m0 0h11.25m-11.25 0V8.25" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_DOWN_LEFT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;