use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 11.25-3-3m0 0-3 3m3-3v7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP_CIRCLE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("data-slot" , "icon") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;