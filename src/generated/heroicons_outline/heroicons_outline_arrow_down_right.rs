use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4.5 4.5 15 15m0 0V8.25m0 11.25H8.25" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("stroke-width" , "1.5")] } ;