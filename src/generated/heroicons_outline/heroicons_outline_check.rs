use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 12.75 6 6 9-13.5" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_CHECK : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;