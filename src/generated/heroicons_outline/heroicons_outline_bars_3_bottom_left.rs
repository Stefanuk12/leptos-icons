use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_3_BOTTOM_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon")] } ;