use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16.49 12 3.75-3.751m0 0-3.75-3.75m3.75 3.75H3.74V19.5" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_UP_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "1.5")] } ;