use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linejoin = "round" d = "M3.75 5.25h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_4 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "1.5") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;