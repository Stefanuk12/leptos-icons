use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M4.499 8.248h15m-15 7.501h15" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_EQUALS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;