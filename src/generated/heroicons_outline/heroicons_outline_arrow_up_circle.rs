use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "m15 11.25-3-3m0 0-3 3m3-3v7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP_CIRCLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("fill" , "none") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;