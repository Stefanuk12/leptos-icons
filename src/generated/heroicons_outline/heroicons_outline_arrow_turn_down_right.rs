use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "m16.49 12 3.75 3.75m0 0-3.75 3.75m3.75-3.75H3.74V4.499" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;