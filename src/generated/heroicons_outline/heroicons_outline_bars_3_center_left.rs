use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" stroke - linecap = "round" d = "M3.75 6.75h16.5M3.75 12H12m-8.25 5.25h16.5" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_3_CENTER_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;