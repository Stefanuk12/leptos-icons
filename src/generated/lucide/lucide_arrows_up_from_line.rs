use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4 6 3-3 3 3" ></ path > < path d = "M7 17V3" ></ path > < path d = "m14 6 3-3 3 3" ></ path > < path d = "M17 17V3" ></ path > < path d = "M4 21h16" ></ path > < / > } } pub const LUCIDE_ARROWS_UP_FROM_LINE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;