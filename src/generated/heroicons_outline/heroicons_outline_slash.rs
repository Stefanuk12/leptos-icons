use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "m9 20.247 6-16.5" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_SLASH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;