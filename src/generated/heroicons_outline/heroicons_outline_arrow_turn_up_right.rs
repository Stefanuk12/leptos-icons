use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m16.49 12 3.75-3.751m0 0-3.75-3.75m3.75 3.75H3.74V19.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_UP_RIGHT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;