use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_3 : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "1.5") , ("fill" , "none") , ("aria-hidden" , "true")] } ;