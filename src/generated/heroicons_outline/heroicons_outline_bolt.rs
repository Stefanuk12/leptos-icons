use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_BOLT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;