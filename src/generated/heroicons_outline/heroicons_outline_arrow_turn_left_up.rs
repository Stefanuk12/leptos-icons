use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M11.99 7.5 8.24 3.75m0 0L4.49 7.5m3.75-3.75v16.499h11.25" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_LEFT_UP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("stroke-width" , "1.5") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;