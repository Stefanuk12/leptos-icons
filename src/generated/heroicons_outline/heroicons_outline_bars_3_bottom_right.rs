use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M3.75 6.75h16.5M3.75 12h16.5M12 17.25h8.25" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_3_BOTTOM_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("stroke" , "currentColor") , ("aria-hidden" , "true") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon")] } ;