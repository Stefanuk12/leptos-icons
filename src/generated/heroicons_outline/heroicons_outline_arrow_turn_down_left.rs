use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m7.49 12-3.75 3.75m0 0 3.75 3.75m-3.75-3.75h16.5V4.499" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_DOWN_LEFT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;