use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" d = "M3.75 9h16.5m-16.5 6.75h16.5" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_2 : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "1.5")] } ;