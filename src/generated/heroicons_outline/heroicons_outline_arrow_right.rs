use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_RIGHT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;