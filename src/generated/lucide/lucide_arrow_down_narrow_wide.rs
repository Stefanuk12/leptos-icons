use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < path d = "M11 4h4" ></ path > < path d = "M11 8h7" ></ path > < path d = "M11 12h10" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_NARROW_WIDE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;