use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 12h14" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true")] } ;