use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "m11.99 16.5 3.75 3.75m0 0 3.75-3.75m-3.75 3.75V3.75H4.49" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_RIGHT_DOWN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;