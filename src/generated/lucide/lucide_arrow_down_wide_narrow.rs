use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < path d = "M11 4h10" ></ path > < path d = "M11 8h7" ></ path > < path d = "M11 12h4" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_WIDE_NARROW : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;