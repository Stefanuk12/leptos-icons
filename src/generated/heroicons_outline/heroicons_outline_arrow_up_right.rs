use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 19.5 15-15m0 0H8.25m11.25 0v11.25" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_UP_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;