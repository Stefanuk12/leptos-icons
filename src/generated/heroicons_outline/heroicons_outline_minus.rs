use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 12h14" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_MINUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("fill" , "none")] } ;