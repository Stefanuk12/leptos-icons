use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "m11.99 7.5 3.75-3.75m0 0 3.75 3.75m-3.75-3.75v16.499H4.49" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_TURN_RIGHT_UP : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "none") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;