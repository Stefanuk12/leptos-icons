use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m16.49 12 3.75 3.75m0 0-3.75 3.75m3.75-3.75H3.74V4.499" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;