use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_LONG_RIGHT : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "none") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5")] } ;