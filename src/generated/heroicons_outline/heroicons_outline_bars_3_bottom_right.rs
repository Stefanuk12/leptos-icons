use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.75 6.75h16.5M3.75 12h16.5M12 17.25h8.25" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_BARS_3_BOTTOM_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;