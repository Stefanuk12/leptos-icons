use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m4.5 12.75 6 6 9-13.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHECK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("fill" , "none") , ("aria-hidden" , "true")] } ;