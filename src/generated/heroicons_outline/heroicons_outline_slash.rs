use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 20.247 6-16.5" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_SLASH : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("aria-hidden" , "true")] } ;