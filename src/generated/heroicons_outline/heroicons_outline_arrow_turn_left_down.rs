use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m11.99 16.5-3.75 3.75m0 0L4.49 16.5m3.75 3.75V3.75h11.25" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_LEFT_DOWN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;