use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_ARROW_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;